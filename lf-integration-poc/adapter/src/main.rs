#![allow(dead_code)]
mod markdown_text_merge;
mod width;
mod line_truncation;
mod quicklook;
mod image_decode;
mod sixel;
mod image_preview;
mod writer_adapter;
use markdown_text_merge::DecodedTextMerge;
use pulldown_cmark::{Parser,Options,Event,Tag,TagEnd};
use ratatui::text::Line;
use std::{env,fs,io::{self,Read,Write},path::Path};
fn safe(s:&str)->String {s.chars().map(|c|if c.is_control() && c!='\n' && c!='\t' {'�'} else if c=='\t' {' '} else {c}).collect()}
fn md(s:&str)->String{
 let mut out=String::new();
 for (e,_) in DecodedTextMerge::new(Parser::new_ext(s,Options::ENABLE_TABLES|Options::ENABLE_TASKLISTS).into_offset_iter()){
  match e{
   Event::Start(Tag::Heading{..})=>out.push_str("◆ "),
   Event::Start(Tag::Item)=>out.push_str("• "),
   Event::Start(Tag::TableRow|Tag::TableHead)=>out.push_str("| "),
   Event::End(TagEnd::TableCell)=>out.push_str(" | "),
   Event::End(TagEnd::TableRow|TagEnd::TableHead|TagEnd::Heading(_)|TagEnd::Paragraph|TagEnd::Item|TagEnd::CodeBlock)=>out.push('\n'),
   Event::Text(t)|Event::Code(t)=>out.push_str(&t),
   Event::SoftBreak|Event::HardBreak=>out.push('\n'),
   Event::TaskListMarker(b)=>out.push_str(if b{"☑ "}else{"☐ "}),
   _=>{}
  }
 }
 safe(&out)
}
fn preview(path:&Path,w:usize,h:usize)->io::Result<String>{
 let m=fs::metadata(path)?;
 if !m.is_file(){return Ok("Not a regular file".into());}
 let ext=path.extension().and_then(|s|s.to_str()).unwrap_or("").to_ascii_lowercase();
 if matches!(ext.as_str(),"pdf"|"doc"|"docx"|"xls"|"xlsx"|"ppt"|"pptx"|"rtf"|"epub"|"pages"|"numbers"|"key"|"png"|"jpg"|"jpeg"|"gif"|"webp"|"heic"|"svg"|"mp4"|"mov"|"mp3"|"m4a"|"zip"){
  let s=format!("{} file\nSize: {} bytes\nSpace: Quick Look",ext.to_uppercase(),m.len());
  return Ok(s.lines().take(h).map(|s|line_truncation::truncate_line_with_ellipsis_if_overflow(Line::from(s.to_string()),w).spans.iter().map(|s|s.content.as_ref()).collect::<String>()).collect::<Vec<_>>().join("\n"))
 }
 let mut b=Vec::new();fs::File::open(path)?.take(1024*1024).read_to_end(&mut b)?;
 if (ext=="md"||ext=="markdown") && let Ok(s)=std::str::from_utf8(&b) && !s.contains('\0') {
   return Ok(writer_adapter::render(s,w,h,path.parent().unwrap_or(Path::new(".")),true))
 }
 let s=match std::str::from_utf8(&b){
  Ok(s) if !s.contains('\0')=>if ext=="md"||ext=="markdown"{md(s)}else{safe(s)},
  _=>format!("Binary: {} bytes\nSpace: Quick Look",m.len())
 };
 let out=s.lines().take(h).map(|s|line_truncation::truncate_line_with_ellipsis_if_overflow(Line::from(s.to_string()),w).spans.iter().map(|s|s.content.as_ref()).collect::<String>()).collect::<Vec<_>>().join("\n");
 Ok(out)
}
fn main()->io::Result<()>{
 let a:Vec<_>=env::args_os().collect();
 if a.get(1).is_some_and(|x|x=="quicklook") {
  if a.len()==3 {return quicklook::open(Path::new(&a[2]),false)}
  if a.len()==4 && a[2]=="--dry-run" {return quicklook::open(Path::new(&a[3]),true)}
  eprintln!("Usage: lfpreview quicklook [--dry-run] PATH");std::process::exit(2)
 }
 if a.len()!=7 {eprintln!("Usage: lfpreview PATH WIDTH HEIGHT X Y MODE");std::process::exit(2)}
 let num=|i:usize|a[i].to_string_lossy().parse::<usize>().unwrap_or(1).clamp(1,1000);
 let path=Path::new(&a[1]);let w=num(2);let h=num(3);
 let mode=a[6].to_string_lossy();
 if mode!="preview"&&mode!="preload"{std::process::exit(2)}
 let out=if image_preview::supported(path) {
   image_preview::preview(path,w,h).unwrap_or_else(|_|preview(path,w,h).unwrap_or_else(|_|"Cannot preview image".into()))
 }else{preview(path,w,h).unwrap_or_else(|_|"Cannot preview file".into())};
 if let Some(log)=env::var_os("LFP_POC_TRACE"){
   use std::fs::OpenOptions;let mut f=OpenOptions::new().create(true).append(true).open(log)?;
   writeln!(f,"path={:?} width={} height={} x={:?} y={:?} mode={} output={:?}",path,w,h,a[4],a[5],mode,out)?;
 }
 let mut stdout=io::stdout().lock();
 match writeln!(stdout,"{out}"){Err(e) if e.kind()==io::ErrorKind::BrokenPipe=>Ok(()),r=>r}
}
#[test] fn escapes_are_removed(){assert!(!safe("a\x1b]52;x\x07").contains('\x1b'));assert!(!md("# hi\x1b[31m").contains('\x1b'));}
#[test] fn md_renders(){let s=md("# 中文\n\n- item &amp; x\n\n| a | b |\n|---|---|\n| x | y |\n");assert!(s.contains("◆ 中文"));assert!(s.contains("• item & x"));assert!(s.contains("| x | y |"));}


//! Safe terminal adapter for the extracted Codex Markdown Writer.
use ratatui::{style::{Color,Modifier,Style},text::Line};
use std::{fmt::Write,path::Path};
fn color(c:Color,fg:bool,out:&mut String){
 let base=if fg{38}else{48};
 let idx=match c{Color::Black=>0,Color::Red=>1,Color::Green=>2,Color::Yellow=>3,Color::Blue=>4,Color::Magenta=>5,Color::Cyan=>6,Color::Gray=>7,Color::DarkGray=>8,Color::LightRed=>9,Color::LightGreen=>10,Color::LightYellow=>11,Color::LightBlue=>12,Color::LightMagenta=>13,Color::LightCyan=>14,Color::White=>15,
 Color::Rgb(r,g,b)=>{let _=write!(out,";{base};2;{r};{g};{b}");return},
 Color::Indexed(i)=>{let _=write!(out,";{base};5;{i}");return},
 Color::Reset=>return};
 let _=write!(out,";{base};5;{idx}");
}
fn sgr(style:Style,out:&mut String){
 out.push_str("\x1b[0");
 if let Some(c)=style.fg{color(c,true,out)}
 if let Some(c)=style.bg{color(c,false,out)}
 for (m,v) in [(Modifier::BOLD,1),(Modifier::DIM,2),(Modifier::ITALIC,3),(Modifier::UNDERLINED,4),(Modifier::REVERSED,7),(Modifier::CROSSED_OUT,9)]{
  if style.add_modifier.contains(m){let _=write!(out,";{v}");}
 }
 out.push('m');
}
fn clean(s:&str)->String{
 s.chars().map(|c|if c=='\t'{' '}else if c.is_control(){'�'}else{c}).collect()
}
pub fn render(input:&str,width:usize,height:usize,cwd:&Path,styled:bool)->String{
 // Keep newlines/tabs for the Markdown parser, never emit source control sequences.
 let sanitized:String=input.chars().map(|c|if c.is_control()&&c!='\n'&&c!='\r'&&c!='\t'{'�'}else{c}).collect();
 let lines=lf_preview_poc::render(&sanitized,width,cwd);
 let mut out=String::new();
 for (i,line) in lines.into_iter().take(height).enumerate(){
  if i>0{out.push('\n')}
  let l:Line<'static>=crate::line_truncation::truncate_line_with_ellipsis_if_overflow(line,width);
  for span in l.spans{
   if styled{sgr(l.style.patch(span.style),&mut out)}
   out.push_str(&clean(&span.content));
  }
  if styled{out.push_str("\x1b[0m")}
 }
 out
}
#[test] fn native_writer(){
 let s=render("# 中文\n\n- first second third fourth",14,20,Path::new("."),false);
 assert!(s.contains("# 中文"));assert!(s.contains("• first second\n  third fourth"));
 let s=render("**bold** \x1b]52;x\x07",30,20,Path::new("."),true);
 assert!(s.contains("\x1b[0;"));assert!(!s.contains("\x1b]"));
}
#[test] fn narrow_table(){
 let input="| File | Status | Description |\n|---|---|---|\n| README.md | PASS | long narrative to force width adjustment |\n| src/main.rs | TODO | code sample |";
 let wide=render(input,72,100,Path::new("."),false);
 assert!(wide.contains('━'));assert!(wide.contains("README.md"));
 let narrow=render(input,24,100,Path::new("."),false);
 assert!(narrow.contains("Description"));assert!(narrow.contains("README.md"));
 assert!(narrow.lines().all(|l|crate::width::display_width(l)<=24));
}


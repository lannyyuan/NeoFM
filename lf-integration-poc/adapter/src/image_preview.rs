//! lf preview adapter using Codex's image decoder and Sixel encoder.
use anyhow::{Context,Result,bail};
use image::{DynamicImage,ImageFormat,ImageReader,Limits,RgbaImage};
use std::{env,fs,io::{Cursor,Read},path::Path};
const MAX_FILE:u64=16*1024*1024;
const MAX_FRAME:usize=512*1024;
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Backend {Sixel,Ansi,None}
fn choose(explicit:&str,term:&str,program:&str,mux:bool)->Backend{
 match explicit {
  "none"=>Backend::None,"ansi"=>Backend::Ansi,"sixel"=>if mux{Backend::Ansi}else{Backend::Sixel},
  _=>if !mux&&((program=="iTerm.app"||program=="WindowsTerminal")||term.contains("sixel")||term.starts_with("foot")||term.contains("mlterm")){Backend::Sixel}else{Backend::Ansi}
 }
}
pub fn backend()->Backend{
 let program=if cfg!(windows)&&env::var_os("WT_SESSION").is_some() {"WindowsTerminal".to_owned()}else{env::var("TERM_PROGRAM").unwrap_or_default()};
 choose(&env::var("LFP_IMAGE_BACKEND").unwrap_or_default(),&env::var("TERM").unwrap_or_default(),&program,env::var_os("TMUX").is_some()||env::var_os("STY").is_some())
}
pub fn supported(path:&Path)->bool{
 path.extension().and_then(|x|x.to_str()).and_then(ImageFormat::from_extension).is_some_and(supported_format)
}
fn supported_format(f:ImageFormat)->bool{matches!(f,ImageFormat::Png|ImageFormat::Jpeg|ImageFormat::Gif|ImageFormat::WebP)}
fn decode(p:&Path)->Result<(DynamicImage,String)>{
 let m=fs::metadata(p)?;
 if !m.is_file()||m.len()>MAX_FILE{bail!("Image exceeds file limit or is not regular")}
 let mut bytes=Vec::new();fs::File::open(p)?.take(MAX_FILE+1).read_to_end(&mut bytes)?;
 if bytes.len() as u64>MAX_FILE{bail!("Image exceeds file limit")}
 let r=ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
 let f=r.format().filter(|f|supported_format(*f)).context("Unsupported image format")?;
 let mut limits=Limits::default(); limits.max_alloc=Some(32*1024*1024);limits.max_image_width=Some(4096);limits.max_image_height=Some(4096);
 let img=crate::image_decode::decode_oriented(r,limits)?;
 let label=format!("{} · {} × {}",f.extensions_str()[0].to_ascii_uppercase(),img.width(),img.height());
 let img=if img.width()>768||img.height()>768 {img.thumbnail(768,768)}else{img};
 Ok((img,label))
}
// Use the same terminal window size as lf, without consuming input or writing to tty.
fn cell_dimensions()->(u32,u32){
 #[cfg(unix)] {
  use std::os::fd::AsRawFd;
  if let Ok(f)=fs::File::open("/dev/tty"){
   let mut ws:libc::winsize=unsafe{std::mem::zeroed()};
   let rc=unsafe{libc::ioctl(f.as_raw_fd(),libc::TIOCGWINSZ,&mut ws)};
   if rc==0&&ws.ws_col>0&&ws.ws_row>0&&ws.ws_xpixel>0&&ws.ws_ypixel>0{
    let (cw,ch)=(u32::from(ws.ws_xpixel)/u32::from(ws.ws_col),u32::from(ws.ws_ypixel)/u32::from(ws.ws_row));
    if cw>0&&ch>0{return (cw,ch)}
   }
  }
 }
 (10,20)
}
fn fit(img:&DynamicImage,w:u32,h:u32)->RgbaImage{
 // thumbnail maintains aspect ratio; never enlarge native raster.
 let (w,h)=(w.max(1).min(img.width()),h.max(1).min(img.height()));
 img.thumbnail(w,h).to_rgba8()
}
fn label_width(s:&str,w:usize)->String{
 crate::line_truncation::truncate_line_with_ellipsis_if_overflow(ratatui::text::Line::from(s.to_owned()),w).spans.iter().map(|s|s.content.as_ref()).collect()
}
fn flatten(px:&image::Rgba<u8>)->[u8;3]{
 let a=u16::from(px[3]);let c=|v:u8|((u16::from(v)*a+24*(255-a))/255) as u8;
 [c(px[0]),c(px[1]),c(px[2])]
}
fn ansi(img:&DynamicImage,w:usize,h:usize)->String{
 let p=fit(img,w.min(100) as u32,(h.min(45)*2) as u32);
 let mut out=String::new();
 use std::fmt::Write;
 for y in (0..p.height()).step_by(2){
  for x in 0..p.width(){
   let [r,g,b]=flatten(p.get_pixel(x,y));
   let [r2,g2,b2]=flatten(p.get_pixel(x,(y+1).min(p.height()-1)));
   let _=write!(out,"\x1b[38;2;{r};{g};{b};48;2;{r2};{g2};{b2}m▀");
  }
  out.push_str("\x1b[0m\n");
 }
 out
}
fn render(img:&DynamicImage,label:&str,w:usize,h:usize,b:Backend,cell:(u32,u32))->Result<String>{
 if h<2||w<2||b==Backend::None{return Ok(label_width(label,w))}
 let label=label_width(label,w);
 if b==Backend::Sixel{
  let mut px=fit(img,(w as u32*cell.0).min(384),((h-1) as u32*cell.1).min(288));
  let mut frame=crate::sixel::encode_rgba(px.as_raw(),px.width(),px.height())?;
  if frame.len()>MAX_FRAME{
   px=fit(img,160.min(px.width()),120.min(px.height()));
   frame=crate::sixel::encode_rgba(px.as_raw(),px.width(),px.height())?;
  }
  if frame.len()<=MAX_FRAME{return Ok(format!("{label}\n{}",String::from_utf8(frame)?))}
 }
 Ok(format!("{label}\n{}",ansi(img,w,h-1)))
}
pub fn preview(p:&Path,w:usize,h:usize)->Result<String>{
 let (img,label)=decode(p)?;
 render(&img,&label,w,h,backend(),cell_dimensions())
}
#[test] fn routing(){
 assert_eq!(choose("","xterm-256color","iTerm.app",false),Backend::Sixel);
 assert_eq!(choose("","xterm-256color","WindowsTerminal",false),Backend::Sixel);
 assert_eq!(choose("","xterm-256color","Apple_Terminal",false),Backend::Ansi);
 assert_eq!(choose("sixel","xterm","",true),Backend::Ansi);
 assert_eq!(choose("none","foot","",false),Backend::None);
}
#[test] fn fits_and_bounds(){
 let img=DynamicImage::ImageRgba8(RgbaImage::from_pixel(1000,500,image::Rgba([255,0,0,255])));
 let s=render(&img,"PNG",20,6,Backend::Sixel,(10,20)).unwrap();
 assert!(s.contains("\x1bP9;1;0q\"1;1;200;100"));assert!(s.ends_with("\x1b\\"));assert!(s.len()<MAX_FRAME);
 let a=render(&img,"PNG",20,6,Backend::Ansi,(10,20)).unwrap();assert!(a.contains("▀"));assert!(!a.contains("\x1bP"));
 assert_eq!(render(&img,"PNG",20,1,Backend::Sixel,(10,20)).unwrap(),"PNG");
}
#[test] fn real_formats_and_invalid(){
 use image::{ImageFormat,RgbImage};
 let dir=std::env::temp_dir().join(format!("lfpreview-image-test-{}",std::process::id()));fs::create_dir_all(&dir).unwrap();
 let img=DynamicImage::ImageRgb8(RgbImage::from_fn(32,24,|x,y|image::Rgb([(x*7) as u8,(y*10) as u8,110])));
 for (ext,f) in [("png",ImageFormat::Png),("jpg",ImageFormat::Jpeg),("gif",ImageFormat::Gif),("webp",ImageFormat::WebP)]{
  let p=dir.join(format!("sample.{ext}"));img.save_with_format(&p,f).unwrap();let (decoded,_)=decode(&p).unwrap();assert_eq!((decoded.width(),decoded.height()),(32,24));
 }
 let invalid=dir.join("bad.png");fs::write(&invalid,b"not an image").unwrap();assert!(decode(&invalid).is_err());fs::remove_dir_all(dir).unwrap();
}


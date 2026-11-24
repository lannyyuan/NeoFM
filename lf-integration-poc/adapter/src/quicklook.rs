use std::{env,fs,io::{self,IsTerminal,Write},path::Path,process::{Command,Stdio}};
#[derive(Debug,PartialEq,Eq)]
pub(crate) enum Backend { Mac, Linux, Terminal }
pub(crate) fn choose(os:&str,ssh:bool,gui:bool)->Backend {
 if ssh {return Backend::Terminal;}
 match os {"macos" => Backend::Mac,"linux" if gui => Backend::Linux,_=>Backend::Terminal}
}
pub(crate) fn backend()->Backend {
 choose(env::consts::OS,env::var_os("SSH_CONNECTION").is_some()||env::var_os("SSH_TTY").is_some()||env::var_os("SSH_CLIENT").is_some(),env::var_os("DISPLAY").is_some()||env::var_os("WAYLAND_DISPLAY").is_some())
}
fn trace(backend:&Backend,path:&Path)->io::Result<()> {
 if let Some(log)=env::var_os("LFP_QL_TRACE"){
  let mut f=fs::OpenOptions::new().create(true).append(true).open(log)?;
  writeln!(f,"backend={backend:?} path={path:?}")?;
 }
 Ok(())
}
pub(crate) fn open(path:&Path,dry:bool)->io::Result<()> {
 let p=fs::canonicalize(path)?;
 if !fs::metadata(&p)?.is_file(){return Err(io::Error::new(io::ErrorKind::InvalidInput,"Quick Look requires a regular file"));}
 let b=backend();trace(&b,&p)?;
 if dry||env::var_os("LFP_QL_DRY_RUN").as_deref()==Some(std::ffi::OsStr::new("1")){
  println!("Quick Look: {b:?} {}",crate::safe(&p.to_string_lossy()));return Ok(());
 }
 match b {
  Backend::Mac=>{
   let st=Command::new("/usr/bin/qlmanage").arg("-p").arg(&p).status()?;
   if st.success(){return Ok(())} eprintln!("Quick Look exited: {st}");terminal(&p)
  },
  Backend::Linux=>{
   match Command::new("dzurkel").arg("--show").arg(&p).status(){
    Ok(s) if s.success()=>Ok(()),
    _=>{eprintln!("Dzurkel unavailable or failed; falling back to text");terminal(&p)}
   }
  },
  Backend::Terminal=>terminal(&p)
 }
}
fn terminal(p:&Path)->io::Result<()>{
 let mut body=crate::preview(p,100,1000)?;
 if p.extension().and_then(|s|s.to_str()).is_some_and(|x|x.eq_ignore_ascii_case("pdf")){
   body=format!("PDF: {} bytes\nNo graphical Quick Look in this SSH session. PDF terminal reader is not implemented yet.",fs::metadata(p)?.len());
 }
 if !io::stdout().is_terminal(){println!("{body}");return Ok(());}
 match Command::new("less").args(["-R","-X"]).stdin(Stdio::piped()).spawn(){
  Ok(mut child)=>{if let Some(mut stdin)=child.stdin.take(){let _=stdin.write_all(body.as_bytes());}let _=child.wait()?;Ok(())},
  Err(_)=>{println!("{body}");Ok(())}
 }
}
#[test] fn routes(){
 assert_eq!(choose("macos",false,false),Backend::Mac);
 assert_eq!(choose("macos",true,true),Backend::Terminal);
 assert_eq!(choose("linux",false,true),Backend::Linux);
 assert_eq!(choose("linux",true,true),Backend::Terminal);
 assert_eq!(choose("linux",false,false),Backend::Terminal);
}


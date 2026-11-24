using System;
using System.Runtime.InteropServices;
[ComImport,Guid("43826D1E-E718-42EE-BC55-A1E261C37BFE"),InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IShellItem {}
[ComImport,Guid("947AAB5F-0A5C-4C13-B4D6-4BF7836FC9F8"),InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
interface IFileOperation {
 void Advise(IntPtr sink, out uint cookie);
 void Unadvise(uint cookie);
 void SetOperationFlags(uint flags);
 void SetProgressMessage([MarshalAs(UnmanagedType.LPWStr)] string message);
 void SetProgressDialog(IntPtr dialog);
 void SetProperties(IntPtr props);
 void SetOwnerWindow(UIntPtr hwnd);
 void ApplyPropertiesToItem(IShellItem item);
 void ApplyPropertiesToItems(IntPtr items);
 void RenameItem(IShellItem item,[MarshalAs(UnmanagedType.LPWStr)] string name,IntPtr sink);
 void RenameItems(IntPtr items,[MarshalAs(UnmanagedType.LPWStr)] string name);
 void MoveItem(IShellItem item,IShellItem dest,[MarshalAs(UnmanagedType.LPWStr)] string name,IntPtr sink);
 void MoveItems(IntPtr items,IShellItem dest);
 void CopyItem(IShellItem item,IShellItem dest,[MarshalAs(UnmanagedType.LPWStr)] string name,IntPtr sink);
 void CopyItems(IntPtr items,IShellItem dest);
 void DeleteItem(IShellItem item,IntPtr sink);
 void DeleteItems(IntPtr items);
 void NewItem(IShellItem dest,uint attrs,[MarshalAs(UnmanagedType.LPWStr)] string name,[MarshalAs(UnmanagedType.LPWStr)] string template,IntPtr sink);
 void PerformOperations();
 void GetAnyOperationsAborted([MarshalAs(UnmanagedType.Bool)] out bool aborted);
}
public static class LFRecycle {
 [DllImport("shell32.dll",CharSet=CharSet.Unicode,PreserveSig=false)]
 static extern void SHCreateItemFromParsingName(string path,IntPtr bind,ref Guid iid,[MarshalAs(UnmanagedType.Interface)] out IShellItem item);
 public static void Move(string path) {
  var type=Type.GetTypeFromCLSID(new Guid("3AD05575-8857-4850-9277-11B85BDB8E09"),true);
  var op=(IFileOperation)Activator.CreateInstance(type);
  IShellItem item=null;
  try {
   // RECYCLEONDELETE | WANTNUKEWARNING | EARLYFAILURE; no NOCONFIRMATION.
   op.SetOperationFlags(0x00080000u | 0x00004000u | 0x00100000u | 0x00000040u);
   Guid iid=typeof(IShellItem).GUID;
   SHCreateItemFromParsingName(path,IntPtr.Zero,ref iid,out item);
   op.DeleteItem(item,IntPtr.Zero);
   op.PerformOperations();
   bool aborted;op.GetAnyOperationsAborted(out aborted);
   if(aborted)throw new InvalidOperationException("Recycle operation aborted");
  } finally {
   if(item!=null)Marshal.ReleaseComObject(item);
   Marshal.ReleaseComObject(op);
  }
 }
}

> # Introduction to Windows API - Tryhackme

# Summary

## Task 2 - Subsystem and Hardware Interaction
1. Does a process in the user mode have direct hardware access?<br>
    > User mode: No direct hardware access

    **Answer:** N

1. Does launching an application as an administrator open the process in kernel mode? (Y/N)<br>
    > Administrator is also just a user.<br>

    **Answer:** N

## Task 3 - Components of the Windows API
1. What header file imports and defines the User32 DLL and structure?<br>
    Research on Google, I found the link [https://forums.ni.com/t5/LabVIEW/Where-can-i-find-the-User32-dll-header-files/td-p/937780](https://forums.ni.com/t5/LabVIEW/Where-can-i-find-the-User32-dll-header-files/td-p/937780), he refer to `winuser.h` and `windows.h`. Then, I found a link in wiki [https://en.wikipedia.org/wiki/Windows.h](https://en.wikipedia.org/wiki/Windows.h).<br>
    > winuser.h – user32.dll: user services

    **Answer:** winuser.h

1. What parent header file contains all other required child and core header files?<br>
    **Answer:** windows.h

## Task 4 - OS Libraries
1. What overarching namespace provides P/Invoke to .NET?<br>
    From C# example code.<br>
    **Answer:** System

1. What memory protection solution obscures the process of importing API calls?<br>
    > The process of obtaining pointers to these functions is obscured because of ASLR (Address Space Layout Randomization) implementations

    **Answer:** ASLR

## Task 5 - API Call Structure
1. Which character appended to an API call represents an ANSI encoding?<br>
    **Answer:** A

1. Which character appended to an API call represents extended functionality?<br>
    **Answer:** Ex

1. What is the memory allocation type of 0x00080000 in the VirtualAlloc API call?<br>
    Research on Windows documentation [https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc](https://docs.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtualalloc), VirtualAlloc have four parameters in which `flAllocationType` has `MEM_RESET` when has the value 0x00080000.<br>
    **Answer:** MEM_RESET

## Task 6 - C API Implementations
1. Do you need to define a structure to use API calls in C? (Y/N)<br>
    At first line of documentation.<br>
    > Microsoft provides low-level programming languages such as C and C++ with a pre-configured set of libraries that we can use to access needed API calls.

    **Answer:** N

## Task 7 - .NET and PowerShell API Implementations
1. What method is used to import a required DLL?<br>
    From .NET example code.<br>
    **Answer:** DllImport

1. What type of method is used to reference the API call to obtain a struct?<br>
    Because function which we use is defined in DLL library so we only need to import this DLL then call it as external function.<br>
    **Answer:** external

## Task 8 - Commonly Abused API Calls
1. Which API call returns the address of an exported DLL function?<br>
    > GetProcAddress: Returns the address of a specified exported DLL  function

    **Answer:** GetProcAddress

1. Which API call imports a specified DLL into the address space of the calling process?<br>
    > LoadLibraryA: Maps a specified DLL  into the address space of the calling process

    **Answer:** LoadLibraryA

## Task 9 - Malware Case Study
```C#
public static void Main() {
	_hookID = SetHook(_proc);
	Application.Run();
	UnhookWindowsHookEx(_hookID);
	Application.Exit();
}

private static IntPtr SetHook(LowLevelKeyboardProc proc) {
	using (Process curProcess = Process.GetCurrentProcess()) {
		return SetWindowsHookEx(WHKEYBOARDLL, proc, GetModuleHandle(curProcess.ProcessName), 0);
	}
}
```

1. What Win32 API call is used to obtain a pseudo handle of our current process in the keylogger sample?<br>
    **Answer:** GetCurrentProcess

1. What Win32 API call is used to set a hook on our current process in the keylogger sample?<br>
    > SetWindowsHookEx: Installs a memory hook into a hook chain to monitor for certain events

    **Answer:** SetWindowsHookEx

1. What Win32 API call is used to obtain a handle from the pseudo handle in the keylogger sample?<br>
    **Answer:** GetModuleHandle

1. What Win32 API call is used unset the hook on our current process in the keylogger sample?<br>
    > UnhookWindowsHookEx is used to remove a hook procedure installed in a hook chain by the SetWindowsHookEx function.
    
    **Answer:** UnhookWindowsHookEx

```C#
UInt32 funcAddr = VirtualAlloc(0, (UInt32)shellcode.Length, MEM_COMMIT, PAGE_EXECUTE_READWRITE);
Marshal.Copy(shellcode, 0, (IntPtr)(funcAddr), shellcode.Length);
IntPtr hThread = IntPtr.Zero;
UInt32 threadId = 0;
IntPtr pinfo = IntPtr.Zero;
hThread = CreateThread(0, 0, funcAddr, pinfo, 0, ref threadId);
WaitForSingleObject(hThread, 0xFFFFFFFF);
return;
```
1. What Win32 API call is used to allocate memory for the size of the shellcode in the shellcode launcher sample?<br>
    > VirtualAlloc: Reserves, commits, or changes the state of a region of pages in the virtual address space of the calling process.
    **Answer:** VirtualAlloc

1. What native method is used to write shellcode to an allocated section of memory in the shellcode launcher sample?<br>
    > Marshal.Copy: Copies data from a managed array to an unmanaged memory pointer, or from an unmanaged memory pointer to a managed array.

    **Answer:** Marshal.Copy

1. What Win32 API call is used to create a new execution thread in the shellcode launcher sample?<br>
    > CreateThread: Creates a thread to execute within the virtual address space of the calling process

    **Answer:** CreateThread

1. What Win32 API call is used to wait for the thread to exit in the shellcode launcher sample?<br>
    > WaitForSingleObject: Waits until the specified object is in the signaled state or the time-out interval elapses
    **Answer:** WaitForSingleObject


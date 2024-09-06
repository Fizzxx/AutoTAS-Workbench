use std::{ fmt, mem, ptr, vec };
use std::ffi::CString;
use std::sync::{ Arc, Mutex, MutexGuard };

extern crate winapi;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef;
use winapi::um::{ 
    errhandlingapi, 
    handleapi, 
    libloaderapi, 
    memoryapi, 
    minwinbase, 
    processthreadsapi, 
    winnt
};

use auto_tas::{ egui, View };


pub struct CenterPanel {
    active_processes: Arc<Mutex< Vec<(String, u32)> >>,
    chosen_process: Arc<Mutex< Option<(String, u32)> >>,
    updating_processes: Arc<Mutex< bool >>,
    injection_complete: Arc<Mutex< bool >>,
}

impl Default for CenterPanel {
    fn default() -> Self { 
        Self {
            active_processes: Arc::new(Mutex::new(vec![])),
            chosen_process: Arc::new(Mutex::new(None)),
            updating_processes: Arc::new(Mutex::new(false)),
            injection_complete: Arc::new(Mutex::new(false)),  // does this need to be wrapped?
        } 
    }
}

impl View for CenterPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        if self.chosen_process.lock().unwrap().is_none() {
            self.display_available_processes(ui);
        }
        else if !unsafe{ self.process_still_active() } {
            let injection_complete_clone = self.injection_complete.clone();
            let mut injection_complete = injection_complete_clone.lock().unwrap();
            *injection_complete = false;

            let chosen_process_clone = self.chosen_process.clone();
            let mut chosen_process = chosen_process_clone.lock().unwrap();
            *chosen_process = None;
        }
        else {
            self.display_screen_capture(ui);
        }
    }
}

impl CenterPanel {
    pub fn show(&mut self, ctx: &egui::Context) {
        let frame = egui::containers::Frame::default()
                                .fill(egui::Color32::from_rgb(0, 0, 0))
                                .inner_margin(egui::Margin{
                                    left: 5.0, right: 0.0, top: 5.0, bottom: 5.0});
        
        egui::CentralPanel::default()
            .frame(frame)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn display_available_processes(&mut self, ui: &mut egui::Ui) {
        use tokio::task;

        let updating_processes_clone = Arc::clone(&self.updating_processes);
        let mut updating_processes = updating_processes_clone.lock().unwrap();

        if !*updating_processes {
            *updating_processes = true;
            let active_processes_clone = Arc::clone(&self.active_processes);
            let updating_processes_clone = Arc::clone(&self.updating_processes);
            task::spawn(async move {
                Self::update_active_processes(active_processes_clone).await;
                *updating_processes_clone.lock().unwrap() = false;
            });
        }

        let active_processes_clone = Arc::clone(&self.active_processes);
        let active_processes = active_processes_clone.lock().unwrap();

        if active_processes.is_empty() { ui.label("no active_processes detected!"); }
        else {
            for (process_name, process_id) in active_processes.iter() {
                if ui.button(process_name.clone()).clicked() {
                    let chosen_process_clone = Arc::clone(&self.chosen_process);
                    let mut chosen_process = chosen_process_clone.lock().unwrap();
                    *chosen_process = Some( (process_name.to_string(), *process_id) );

                    let injection_complete_clone = self.injection_complete.clone();
                    let mut injection_complete = injection_complete_clone.lock().unwrap();
                    unsafe {
                        if self.inject_dll(chosen_process) { *injection_complete = true };
                    }
                }
            }
        }
    }


    // Error checking function for result validation
    fn check_result<T>(result: *mut T, error_message: &str) -> *mut T {
        if result.is_null() {
            eprintln!("{}", error_message);
            std::process::exit(1);
        }
        result
    }

    fn display_screen_capture(&mut self, _ui: &mut egui::Ui) {}

    unsafe fn inject_dll(&mut self, chosen_process: MutexGuard<'_, Option<(String, u32)>>) -> bool {
        let dll_path = r"C:\atdll.dll";
        let process_id = chosen_process.as_ref().unwrap().1;

        let process_handle = Self::check_result(
            processthreadsapi::OpenProcess(winnt::PROCESS_ALL_ACCESS, 0, process_id),
            "Failed to open target process."
        );

        let alloc_size = dll_path.len() + 1;
        let remote_memory = Self::check_result(
            memoryapi::VirtualAllocEx(
                process_handle,
                ptr::null_mut(),
                alloc_size,
                winnt::MEM_COMMIT | winnt::MEM_RESERVE,
                winnt::PAGE_READWRITE),
            "Failed to allocate memory in target process."
        );

        let dll_path_cstr = CString::new(dll_path).unwrap();
        let bytes_written = memoryapi::WriteProcessMemory(
            process_handle,
            remote_memory,
            dll_path_cstr.as_ptr() as minwindef::LPVOID,
            alloc_size,
            ptr::null_mut()
        );

        if bytes_written == 0 {
            eprintln!("Failed to write DLL path into target process memory.");
            handleapi::CloseHandle(process_handle);
            return false;
        }

        // Get the address of LoadLibraryA from kernel32.dll.
        let kernel32_mod_name = CString::new("kernel32.dll").unwrap();
        let kernel32_handle = libloaderapi::GetModuleHandleA(kernel32_mod_name.as_ptr());
        
        let lp_proc_name = CString::new("LoadLibraryA").unwrap();
        let load_library_addr = Self::check_result(
            libloaderapi::GetProcAddress(kernel32_handle, lp_proc_name.as_ptr()),
            "Failed to get address of LoadLibraryA."
        );

        // Create a remote thread in the target process to call LoadLibraryA with the DLL path.
        let remote_thread = processthreadsapi::CreateRemoteThread(
            process_handle,
            ptr::null_mut(),
            0,
            mem::transmute::<_, minwinbase::LPTHREAD_START_ROUTINE>(load_library_addr),
            remote_memory,
            0,
            ptr::null_mut()
        );

        if remote_thread == ptr::null_mut() {
            eprintln!("Failed to create remote thread in target process.");
            handleapi::CloseHandle(process_handle);
            return false;
        }

        // Wait for the remote thread to finish and clean up.
        handleapi::CloseHandle(remote_thread);
        handleapi::CloseHandle(process_handle);

        true
    }

    unsafe fn process_still_active(&self) -> bool {
        let chosen_process_clone = self.chosen_process.clone();
        let chosen_process = chosen_process_clone.lock().unwrap();

        if chosen_process.is_none() { // if this happens, something went very wrong.
            panic!("A process was thought to be chosen, but None was found.");
            // return false; 
        } 
        let pid = chosen_process.as_ref().unwrap().1;

        let handle = processthreadsapi::OpenProcess(
            winnt::PROCESS_QUERY_INFORMATION, 
            minwindef::FALSE, 
            pid);

        if handle.is_null() { return false; }

        let mut exit_code: DWORD = 0;
        let result = processthreadsapi::GetExitCodeProcess(handle, &mut exit_code);

        handleapi::CloseHandle(handle);

        if result == 0 { return false; }

        exit_code == minwinbase::STILL_ACTIVE
    }


    async fn update_active_processes(active_processes: Arc<Mutex< Vec<(String, u32)> >>) {
        use std::{ thread, time };

        match Self::enumerate_active_processes() {
            Ok(new_processes) => {
                let mut change_processes = active_processes.lock().unwrap();
                *change_processes = new_processes;
            }
            Err(e) => {
                panic!("Error enumerating active_processes: {}", e);
            }
        }

        thread::sleep(time::Duration::from_secs(1));
    }

    // -!- currently only locates Steam executables
    fn enumerate_active_processes() -> Result<Vec<(String, u32)>, ProcessEnumerationError> {
        use std::ffi::CStr;
        use std::ptr;
        use winapi::um::tlhelp32::{
            CreateToolhelp32Snapshot, Process32First, Process32Next, 
            PROCESSENTRY32, TH32CS_SNAPPROCESS,
        };
        use winapi::um::psapi::GetModuleFileNameExA;
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
        use winapi::um::winnt::PROCESS_VM_READ;
        use winapi::shared::minwindef::MAX_PATH;
        
        let mut active_processes: Vec<(String, u32)> = vec![];

        unsafe {
            // Take a snapshot of all active_processes
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
            if snapshot == ptr::null_mut() {
                let error_code = errhandlingapi::GetLastError();
                return Err(ProcessEnumerationError::SnapshotFailed(error_code));
            }
    
            let mut process_entry = PROCESSENTRY32 {
                dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
                ..std::mem::zeroed()
            };
    
            // Get the first process
            if Process32First(snapshot, &mut process_entry) == 0 {
                let error_code = errhandlingapi::GetLastError();
                CloseHandle(snapshot);
                return Err(ProcessEnumerationError::FirstProcessFailed(error_code));
            }
    
            // Iterate over all active_processes
            loop {
                let process_id = process_entry.th32ProcessID;
                let h_process = OpenProcess(
                    PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
                    0,
                    process_id,
                );
    
                if h_process != ptr::null_mut() {
                    let mut exe_path = [0u8; MAX_PATH];
                    let len = GetModuleFileNameExA(
                        h_process, 
                        ptr::null_mut(), 
                        exe_path.as_mut_ptr() as *mut i8, 
                        MAX_PATH as u32);
                    if len > 0 {
                        let exe_path_str = CStr::from_ptr(exe_path.as_ptr() as *const i8).to_string_lossy();
                        if exe_path_str.ends_with(".exe") && exe_path_str.contains("Steam\\steamapps") {
                            active_processes.push( (exe_path_str.to_string(), process_id) );
                        }
                    }
                    CloseHandle(h_process);
                } 
                // else {  // prints too much
                //     let error_code = errhandlingapi::GetLastError();
                //     eprintln!("Failed to open process {}: Error code {}", process_id, error_code);
                // }
    
                if Process32Next(snapshot, &mut process_entry) == 0 {
                    break;
                }
            }
            CloseHandle(snapshot);
        }
        return Ok(active_processes);
    }
}

// ------------------------------------------------------------------------------------------
#[derive(Debug)]
enum ProcessEnumerationError {
    SnapshotFailed(DWORD),
    FirstProcessFailed(DWORD),
    // ProcessOpenFailed(DWORD),
}

impl fmt::Display for ProcessEnumerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ProcessEnumerationError::SnapshotFailed(code) => 
                write!(f, "Failed to create snapshot. Error code: {}", code),
            ProcessEnumerationError::FirstProcessFailed(code) => 
                write!(f, "Failed to get first process. Error code: {}", code),
            // ProcessEnumerationError::ProcessOpenFailed(code) => 
            //     write!(f, "Failed to open process. Error code: {}", code),
        }
    }
}

impl std::error::Error for ProcessEnumerationError {}
// ------------------------------------------------------------------------------------------

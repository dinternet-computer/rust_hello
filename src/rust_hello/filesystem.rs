use std::{cell::RefCell, io::{Result, Read, BufRead, BufReader}};
use std::convert::TryInto;

use fatfs::{Write, Seek, SeekFrom};
use ic_cdk_macros::{query, update};

type FileSystem = fatfs::FileSystem<
    fatfs::StdIoWrapper<icfs::StableMemory>,
    icfs_fatfs::TimeProvider,
    fatfs::LossyOemCpConverter,
>;

type Dir<'a> = fatfs::Dir<
    'a,
    fatfs::StdIoWrapper<icfs::StableMemory>,
    icfs_fatfs::TimeProvider,
    fatfs::LossyOemCpConverter,
>;

thread_local! {
    static STABLE_MEMORY: RefCell<icfs::StableMemory> = RefCell::new(icfs::StableMemory::default());
    static FS: RefCell<FileSystem> = {
        let fs: Result<FileSystem> = STABLE_MEMORY.with(|stable_memory| {
            let stable_memory = *stable_memory.borrow();

            let options = fatfs::FsOptions::new()
                .time_provider(icfs_fatfs::TimeProvider::new())
                .update_accessed_date(true);

            let fs = fatfs::FileSystem::new(stable_memory, options)?;
            Ok(fs)
        });
        RefCell::new(fs.unwrap())
    };
}

#[update]
fn init_volume(confirm: Option<String>) -> Option<&'static str> {
    match confirm {
        None => Some("confirm to init_volume ? (input confirm_to_init_volume to init.)"),
        Some(s) => if s == "confirm_to_init_volume" {
            STABLE_MEMORY.with(|stable_memory| {
                #[cfg(target_arch = "wasm32")]
                let memory_pages = core::arch::wasm32::memory_size(0)
                    .try_into()
                    .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error)).unwrap();
    
                #[cfg(not(target_arch = "wasm32"))]
                let memory_pages = 19;
    
                icfs::StableMemory::grow(memory_pages).unwrap();

                fatfs::format_volume(
                    &mut fatfs::StdIoWrapper::from(*stable_memory.borrow()),
                     fatfs::FormatVolumeOptions::new()).unwrap();
            });
            Some("init_volume completed")
        } else {
            Some("confirm to init_volume ? (input confirm_to_init_volume to init.)")
        },
    }
}

fn open_dir_path<'a>(fs: &'a FileSystem, path: &str) -> std::io::Result<Dir<'a>> {
    let root_dir = fs.root_dir();
    let (base_dir_name, sub_dir_path) = path_head_tail(&path)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
    
    match (base_dir_name.as_str(), sub_dir_path.as_str()) {
        (".", "") => Ok(root_dir),
        (".", sub_dir_path) => root_dir
            .open_dir(&sub_dir_path)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error)),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Invalid path: {}", path.to_string()),
        )),
    }   

}

fn path_head_tail(path: &str) -> core::result::Result<(String, String), String> {
    let path_segments = path.split("/").collect::<Vec<_>>();
    let head = path_segments
        .first()
        .ok_or(format!("Invalid path: {}", path.to_string()))?;

    let tail = path_segments[1..].join("/");
    Ok((head.to_string(), tail))
}

fn path_init_last(path: &str) -> core::result::Result<(String, String), String>  {
    let mut path_segments = path.split("/").collect::<Vec<_>>();
    let last = path_segments
        .pop()
        .ok_or(format!("Invalid path: {}", path.to_string()))?;
    
    let init = path_segments.join("/");
    Ok((init, last.to_string()))
}

#[query]
fn cat(path: String) -> String {
    FS.with(|fs| {
        let fs = fs.borrow();
        let (dir_path, file_name) = path_init_last(&path)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        let dir = open_dir_path(&fs, &dir_path)?;
        let mut file = dir.open_file(&file_name)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;

        let contents = String::from_utf8(buf)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        
        std::io::Result::Ok(contents)
    })
    .unwrap()
}

#[query]
fn read_root_size() -> Vec<u64> {
    FS.with(|fs| {
        let fs = fs.borrow();

        let root_dir = fs.root_dir();

        let a = root_dir.iter().map(|d| {
            d.unwrap().len()
        }).collect::<Vec<u64>>();
 
        std::io::Result::Ok(a)
    })
    .unwrap()
}

#[query]
fn read_lines(path: String) -> Vec<String> {
    FS.with(|fs| {
        let fs = fs.borrow();
        let (dir_path, file_name) = path_init_last(&path)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        
        let dir = open_dir_path(&fs, &dir_path)?;
        let mut file = dir.open_file(&file_name)?;
        let v = BufReader::new(&mut file).lines();
        let lines = v.map(|s| s.unwrap()).collect::<Vec<String>>();

        std::io::Result::Ok(lines)
    })
    .unwrap()
}

#[query]
fn cat_at(path: String, at: u64) -> String {
    FS.with(|fs| {
        let fs = fs.borrow();
        let (dir_path, file_name) = path_init_last(&path)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        let dir = open_dir_path(&fs, &dir_path)?;

        let mut file = dir.open_file(&file_name)?;
        file.seek(SeekFrom::Start(at)).unwrap();

        let mut buf = vec![];
        file.read_to_end(&mut buf)?;

        let contents = String::from_utf8(buf)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        std::io::Result::Ok(contents)
    })
    .unwrap()
}

#[query]
fn ls(path: String) -> Vec<String> {
    FS.with(|fs| {
        let fs = fs.borrow();
        let dir = open_dir_path(&fs, &path)?;
        let mut entries = dir
            .iter()
            .map(|entry| {
                entry.map(|e| e.file_name())
                .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))
            })
            .collect::<std::io::Result<Vec<String>>>();

        match entries.as_mut() {
            Ok(ok) => ok.sort(),
            Err(_err) => (),
        }

        entries
    })
    .unwrap()
}

#[update]
fn mkdir(path: String) {
    FS.with(|fs| {
        let fs = fs.borrow();
        let (dir_path, dir_name) = path_init_last(&path)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        
        let dir = open_dir_path(&fs, &dir_path)?;
        dir.create_dir(&dir_name)?;
        std::io::Result::Ok(())
    })
    .unwrap()
}

#[update]
fn rm(path: String) {
    FS.with(|fs| {
        let fs = fs.borrow();
        let (dir_path, target) = path_init_last(&path)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        
        let dir = open_dir_path(&fs, &dir_path)?;
        dir.remove(&target)?;
        std::io::Result::Ok(())
    })
    .unwrap()
}

#[update]
fn write(path: String, contents: String) {
    FS.with(|fs| {
        let fs = fs.borrow();
        let (dir_path, file_name) = path_init_last(&path)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        
        let dir = open_dir_path(&fs, &dir_path)?;
        let mut file = dir.create_file(&file_name)?;
        
        file.seek(SeekFrom::End(0)).unwrap();
        file.write(&contents.into_bytes())?;
        file.flush()?;
        std::io::Result::Ok(())
    })
    .unwrap()
}

#[update]
fn write_file(path: String, contents: String) {
    FS.with(|fs| {
        let fs = fs.borrow();
        let (dir_path, file_name) = path_init_last(&path)
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::Other, error))?;
        let dir = open_dir_path(&fs, &dir_path)?;
        let mut file = dir.create_file(&file_name)?;
        file.truncate()?;
        file.write_all(&contents.into_bytes())?;
        file.flush()?;
        std::io::Result::Ok(())
    })
    .unwrap()
}
use std::fs;
use std::process::Command;

fn main() {
    // Specify the input folder containing the .aac files
    let input_folder = "Put input folder path here";

    // Create an output folder for the converted .mp3 files
    let output_folder = "Put output folder path here";
    fs::create_dir_all(output_folder).expect("Failed to create output folder");

    // Get a list of all .aac files in the input folder
    let aac_files = fs::read_dir(input_folder)
        .expect("Failed to read input folder")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            if let Some(extension) = entry.path().extension() {
                return extension == "aac";
            }
            false
        });

    // Convert each .aac file to .mp3 using FFmpeg
    for aac_file in aac_files {
        let input_file = aac_file.path();
        let output_file = format!(
            "{}/{}.mp3",
            output_folder,
            input_file.file_stem().unwrap().to_string_lossy()
        );

        // Run FFmpeg command to convert the file
        let ffmpeg_output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&input_file)
            .arg(&output_file)
            .output()
            .expect("Failed to execute FFmpeg command");

        if ffmpeg_output.status.success() {
            println!("Converted {} to {}", input_file.display(), output_file);
        } else {
            eprintln!(
                "Failed to convert {}: {}",
                input_file.display(),
                String::from_utf8_lossy(&ffmpeg_output.stderr)
            );
        }
    }
}

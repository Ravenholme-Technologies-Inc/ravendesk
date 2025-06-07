use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;

const NOTE_FILE: &str = ".ravendesk.json";

// --- clap and Note structs (updated Note::new) ---

#[derive(Parser, Debug)]
#[command(author, version, short_flag = 'v', about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new note to the current project
    Add {
        #[arg(required = true, num_args = 1..)]
        text: Vec<String>,
    },
    /// List all notes for the current project
    List, 
    /// Delete a note by its ID
    Delete {
        /// The ID of the note to delete
        id: u64,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    id: u64,
    content: String,
    created_at: u64,
}

impl Note {
    pub fn new(id: u64, content: String) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Note {
            id,
            content,
            created_at,
        }
    }
}

// --- Main application logic ---

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { text } => {
            let note_content = text.join(" ");
            add_note(&note_content)?;
        }
        Commands::List => {
            list_notes()?;
        }
        Commands::Delete { id } => {
            delete_note(*id)?; // We pass the ID to our new function
        }
    }

    Ok(())
}

// --- File I/O and Logic Functions ---

/// Loads notes from the NOTE_FILE. Returns an empty Vec if the file doesn't exist.
fn load_notes() -> Result<Vec<Note>> {
    if !Path::new(NOTE_FILE).exists() {
        return Ok(Vec::new()); // No file, so no notes.
    }

    let file = File::open(NOTE_FILE).context("Failed to open note file")?;
    let reader = BufReader::new(file);

    let notes: Vec<Note> = serde_json::from_reader(reader)
        .context("Failed to parse note file. It might be corrupted.")?;
    
    Ok(notes)
}

/// Saves a slice of notes to the NOTE_FILE, overwriting it.
fn save_notes(notes: &[Note]) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Overwrite the file if it exists
        .open(NOTE_FILE)?;

    let json = serde_json::to_string_pretty(notes)
        .context("Failed to serialize notes to JSON")?;

    file.write_all(json.as_bytes())
        .context("Failed to write to note file")?;

    Ok(())
}

fn add_note(content: &str) -> Result<()> {
    // 1. Load existing notes
    let mut notes = load_notes()?;

    // 2. Generate a new ID (simple timestamp for now)
    let new_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_millis() as u64;

    // 3. Create the new note
    let new_note = Note::new(new_id, content.to_string());
    println!("✅ Added note: {}", new_note.content);

    // 4. Add it to the list and save
    notes.push(new_note);
    save_notes(&notes)?;

    Ok(())
}

fn list_notes() -> Result<()> {
    // 1. Load existing notes
    let notes = load_notes()?;

    // 2. Check if there are any notes to show
    if notes.is_empty() {
        println!("No notes found. Get started by using the 'add' command!");
    } else {
        println!("--- Your Notes ---");
        // 3. Print each note
        for note in notes {
            // We can format this better later
            println!("- (ID: {}) {}", note.id, note.content);
        }
        println!("------------------");
    }
    
    Ok(())
}
fn delete_note(id_to_delete: u64) -> Result<()> {
    // 1. Load the existing notes.
    let mut notes = load_notes()?;

    // 2. Check the number of notes before we try to remove one.
    let initial_note_count = notes.len();

    // 3. Use the `retain` method to keep only the notes whose ID does NOT match.
    // This is an efficient way to remove items from a vector.
    notes.retain(|note| note.id != id_to_delete);

    // 4. Check if a note was actually deleted.
    if notes.len() < initial_note_count {
        // 5. If so, save the modified (smaller) list of notes back to the file.
        save_notes(&notes)?;
        println!("✅ Note with ID {} has been deleted.", id_to_delete);
    } else {
        // 6. If not, inform the user that no note with that ID was found.
        println!("⚠️ No note found with ID {}. No action taken.", id_to_delete);
    }

    Ok(())
}
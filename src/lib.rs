use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct Library {
    // Basic Information
    pub name: String,          // Name of the library
    pub address: String,       // Address of the library
    pub established_year: u32, // Year the library was established
    pub phone_number: String,  // Library contact number
    pub email: String,         // Library email
    pub website: String,       // Website URL of the library
    pub is_open: bool,         // Is the library currently open
    //
    pub books: HashMap<String, Book>, // HashMap of books with ISBN as key

    // Collection Information
    pub num_of_books: u32,             // Total number of books
    pub num_of_ebooks: u32,            // Number of eBooks available
    pub num_of_magazines: u32,         // Number of magazines available
    pub num_of_dvds: u32,              // Number of DVDs available
    pub num_of_audiobooks: u32,        // Number of audiobooks available
    pub available_genres: Vec<String>, // List of genres available

    // Library Services
    pub has_reading_rooms: bool, // Does the library have reading rooms?
    pub has_study_areas: bool,   // Does the library have study areas?
    pub has_wifi: bool,          // Is Wi-Fi available?
    pub open_hours: HashMap<String, String>, // Opening hours by day, e.g., {"Monday": "9:00-17:00"}

    // Staff and Membership
    pub num_of_staff: u32,             // Total number of staff members
    pub staff_names: Vec<String>,      // Names of the library staff
    pub member_count: u32,             // Number of library members
    pub membership_types: Vec<String>, // Types of membership, e.g., "Standard", "Premium"
    pub annual_membership_fees: HashMap<String, f32>, // Membership type and corresponding annual fees

    // Location and Facilities
    pub num_of_floors: u32,            // Number of floors in the library
    pub has_cafeteria: bool,           // Is there a cafeteria?
    pub has_conference_room: bool,     // Is there a conference room available?
    pub num_of_computers: u32,         // Number of computers available for public use
    pub parking_spots: u32,            // Number of parking spots available
    pub nearby_transport: Vec<String>, // List of nearby public transport options

    // Historical Information
    pub historical_events: Vec<String>, // Key historical events in the library's history
    pub notable_books: Vec<String>,     // Notable books in the library's collection
    pub donors: HashMap<String, f32>,   // Donor names and their contribution amounts
}
#[derive(Debug, Clone, Deserialize)]
pub struct Book {
    pub isbn: String,          // ISBN of the book, acts as a unique identifier
    pub title: String,         // Title of the book
    pub author: String,        // Author of the book
    pub publisher: String,     // Publisher of the book
    pub year_published: u32,   // Year the book was published
    pub genre: String,         // Genre of the book
    pub page_count: u32,       // Number of pages
    pub available_copies: u32, // Number of available copies
    pub total_copies: u32,     // Total copies in the library
    pub is_checked_out: bool,  // Whether the book is currently checked out
    pub keywords: Vec<String>, // Keywords related to the book
}

impl Library {
    pub fn new_sample_library() -> Library {
        let json = r#"
            {
    "name": "Central Library",
    "address": "123 Library St.",
    "established_year": 1990,
    "phone_number": "123-456-7890",
    "email": "info@centrallibrary.com",
    "website": "http://www.centrallibrary.com",
    "is_open": true,
    "books": {
        "978-3-16-148410-0": {
            "isbn": "978-3-16-148410-0",
            "title": "Rust Programming",
            "author": "John Doe",
            "publisher": "Tech Press",
            "year_published": 2020,
            "genre": "Programming",
            "page_count": 500,
            "available_copies": 3,
            "total_copies": 5,
            "is_checked_out": false,
            "keywords": ["rust", "programming", "systems"]
        },
        "978-1-23-456789-0": {
            "isbn": "978-1-23-456789-0",
            "title": "Data Structures in Rust",
            "author": "Jane Smith",
            "publisher": "Data Press",
            "year_published": 2022,
            "genre": "Computer Science",
            "page_count": 350,
            "available_copies": 2,
            "total_copies": 4,
            "is_checked_out": true,
            "keywords": ["data structures", "rust"]
        },
        "978-0-12-345678-9": {
            "isbn": "978-0-12-345678-9",
            "title": "Advanced Rust",
            "author": "Alice Johnson",
            "publisher": "Advanced Press",
            "year_published": 2021,
            "genre": "Programming",
            "page_count": 450,
            "available_copies": 1,
            "total_copies": 2,
            "is_checked_out": false,
            "keywords": ["advanced", "rust", "programming"]
        },
        "978-0-13-419144-0": {
            "isbn": "978-0-13-419144-0",
            "title": "The Rust Language Book",
            "author": "The Rust Team",
            "publisher": "Rust Press",
            "year_published": 2019,
            "genre": "Programming",
            "page_count": 600,
            "available_copies": 5,
            "total_copies": 5,
            "is_checked_out": false,
            "keywords": ["rust", "language", "programming"]
        },
        "978-0-321-87758-1": {
            "isbn": "978-0-321-87758-1",
            "title": "Learning Rust",
            "author": "Mark Thompson",
            "publisher": "Learning Press",
            "year_published": 2023,
            "genre": "Programming",
            "page_count": 400,
            "available_copies": 4,
            "total_copies": 4,
            "is_checked_out": false,
            "keywords": ["learning", "rust", "programming"]
        }
    },
    "num_of_books": 5,
    "num_of_ebooks": 10,
    "num_of_magazines": 20,
    "num_of_dvds": 15,
    "num_of_audiobooks": 5,
    "available_genres": ["Programming", "Science Fiction", "Fantasy", "Mystery"],
    "has_reading_rooms": true,
    "has_study_areas": true,
    "has_wifi": true,
    "open_hours": {
        "Monday": "9:00-17:00",
        "Tuesday": "9:00-17:00",
        "Wednesday": "9:00-17:00",
        "Thursday": "9:00-17:00",
        "Friday": "9:00-17:00",
        "Saturday": "10:00-14:00",
        "Sunday": "Closed"
    },
    "num_of_staff": 10,
    "staff_names": ["Alice", "Bob", "Charlie", "David", "Eve"],
    "member_count": 1500,
    "membership_types": ["Standard", "Premium"],
    "annual_membership_fees": {
        "Standard": 25.0,
        "Premium": 50.0
    },
    "num_of_floors": 3,
    "has_cafeteria": true,
    "has_conference_room": true,
    "num_of_computers": 20,
    "parking_spots": 50,
    "nearby_transport": ["Bus", "Train"],
    "historical_events": ["Grand Opening", "First Renovation"],
    "notable_books": ["Rust Programming", "The Rust Language Book"],
    "donors": {
        "John Smith": 1000.0,
        "Mary Johnson": 500.0
    }
}
"#;

        serde_json::from_str(json).unwrap()
    }
}

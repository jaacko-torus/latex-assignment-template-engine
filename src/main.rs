use std::env;
use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
	student: Student,
	class: Class,
	document: Document,
	assignment: Assignment,
	latex: Latex,
}

#[derive(Deserialize)]
struct Student {
	name: String,
	email: String,
	school: String,
	degree: String,
	id: String,
	class: String,
	term: String,
}

#[derive(Deserialize)]
struct Class {
	name: String,
	name_abbr: String,
}

#[derive(Deserialize)]
struct Document {
	date: Option<String>,
}

#[derive(Deserialize)]
struct Assignment {
	id: String,
	name: String,
}

#[derive(Deserialize)]
struct Latex {
	citations: LatexCitations
}

#[derive(Deserialize)]
struct LatexCitations {
	style: String,
	sorting: String,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let config_file = &args[1];
	let document_file = &args[2];
	let output_file = &args[3];
	
	let original = fs::read_to_string(document_file).expect("Something went wrong reading the document file");
	
	let config: Config = toml::from_str(fs::read_to_string(config_file).expect("Something went wrong reading the file").as_str()).unwrap();
	
	let new_file = original
		// student
		.replace("[[student.name]]", config.student.name.as_str())
		.replace("[[student.email]]", config.student.email.as_str())
		.replace("[[student.school]]", config.student.school.as_str())
		.replace("[[student.degree]]", config.student.degree.as_str())
		.replace("[[student.id]]", config.student.id.as_str())
		.replace("[[student.class]]", config.student.class.as_str())
		.replace("[[student.term]]", config.student.term.as_str())
		// class
		.replace("[[class.name]]", config.class.name.as_str())
		.replace("[[class.name_abbr]]", config.class.name_abbr.as_str())
		// assignment
		.replace("[[assignment.id]]", config.assignment.id.as_str())
		.replace("[[assignment.name]]", config.assignment.name.as_str())
		// latex citations
		.replace("[[latex.citations.style]]", config.latex.citations.style.as_str())
		.replace("[[latex.citations.sorting]]", config.latex.citations.sorting.as_str());
	
	fs::write(output_file, new_file).expect("unable to write ti file");
}

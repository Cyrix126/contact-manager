fn edit_x(
    dir_contacts: PathBuf,
    property_edit: &str,
    property_edit_value: &str,
    property_find: PropertyType,
    value_find: &str,
) {
    let all = read_contacts(&dir_contacts);
    if let Some(mut vcard) = find_one_vcard(all, &property_find, &value_find) {
        let mut property = Property::PropertyXName(PropertyXNameData::default(property_edit));
        property
            .set_value(Value::from(ValueTextData::from(property_edit_value)))
            .expect("Unable to set value.");
        vcard
            .set_property(&property)
            .expect("Unable to update property.");
        // écrire le vcard sur le fichier
        let (file, _) = find_file_vcard(&dir_contacts, &vcard);
        fs::write(file, vcard.to_string()).expect("Unable to write file.");
    } else {
        panic!("ne peux pas éditer un contact inexistant !")
    }
}

fn findx(
    book_contacts: PathBuf,
    property_show: &str,
    property_find: PropertyType,
    value_find: String,
) {
    let all = read_contacts(&book_contacts);
    if let Some(vcard) = find_one_vcard(all, &property_find, &value_find) {
        let property = Property::PropertyXName(PropertyXNameData::default(property_show));
        let property_found = vcard.get_property_ref(&property);
        if let Some(property) = property_found {
            let value = property.get_value();
            print!("{value}");
        } else {
            print!("Aucune propriété de ce type.")
        }
    }
}

fn generate_index(dir_book: PathBuf, property1: &PropertyType, property2: &PropertyType) {
    let all = read_contacts(&dir_book);
    let vcards = parse_vcards(&all).unwrap();
    for vcard in vcards {
        let properties1 = properties_by_name(&vcard, &property1);
        let properties2 = properties_by_name(&vcard, &property2);
        if !properties1.is_empty() && !properties2.is_empty() {
            let property1 = properties1[0].get_value().to_string();
            let property2 = properties2[0].get_value().to_string();
            println!("{property1}\t{property2}");
        }
    }
}
fn list(dir_book: PathBuf, property: &PropertyType) {
    let all = read_contacts(&dir_book);
    let vcards = parse_vcards(&all).unwrap();
    for vcard in vcards {
        let properties = properties_by_name(&vcard, property);
        if !properties.is_empty() {
            let property = properties[0].get_value().to_string();
            println!("{property}");
        }
    }
}

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(APP_SHORTNAME).unwrap();
    let mut dir_contacts = xdg_dirs.get_data_home().clone();
    dir_contacts.push("contacts");
    xdg_dirs.create_data_directory("contacts").unwrap();

    let mut dir_books = xdg_dirs.get_data_home().clone();
    dir_books.push("books");

    xdg_dirs.create_data_directory("books").unwrap();

    let args = Cli::parse();
    let book_default_name = String::from("default");
    let book_name: String;
    if let Some(book) = args.book.as_deref() {
        book_name = book.to_string();
    } else {
        book_name = book_default_name;
    };

    //let books = fs::read_dir(&dir_books);

    match args.command {
        Commands::Find {
            //  mut book_name,
            property_show,
            property_find,
            property_value,
            ..
        } => find(
            find_path_book(&dir_books, book_name),
            property_show,
            property_find,
            property_value,
        ),
        Commands::FindInteractive {
            property_show,
            property_find,
            property_value,
            ..
        } => find_interactive(
            find_path_book(&dir_books, book_name),
            property_show,
            property_find,
            &property_value,
        ),
        Commands::FindX {
            //  book_name,
            property_show,
            property_find,
            property_value,
            ..
        } => findx(
            find_path_book(&dir_books, book_name),
            &property_show,
            property_find,
            property_value,
        ),
        Commands::Edit {
            //   book_name,
            property_edit,
            property_edit_value,
            property_find,
            property_value,
            ..
        } => edit(
            find_path_book(&dir_books, book_name),
            property_edit,
            &property_edit_value,
            property_find,
            &property_value,
        ),
        Commands::EditX {
            //   book_name,
            property_edit,
            property_edit_value,
            property_find,
            property_value,
            ..
        } => edit_x(
            find_path_book(&dir_books, book_name),
            &property_edit,
            &property_edit_value,
            property_find,
            &property_value,
        ),
        Commands::New { value_fn, .. } => new(
            dir_contacts,
            find_path_book(&dir_books, book_name),
            &value_fn,
        ),
        Commands::Delete {
            property_find,
            property_value,
            ..
        } => delete(
            find_path_book(&dir_books, book_name),
            dir_contacts,
            property_find,
            property_value,
        ),
        Commands::NewBook { book_value, .. } => newbook(dir_books, book_value),
        Commands::DeleteBook { book_value, .. } => deletebook(dir_books, book_value),
        Commands::Addto {
            book_value,
            property_find,
            property_value,
            ..
        } => addtobook(
            dir_contacts,
            find_path_book(&dir_books, book_value),
            property_find,
            &property_value,
        ),
        Commands::Removefrom {
            book_value,
            property_find,
            property_value,
            ..
        } => removefrombook(dir_books, book_value, property_find, property_value),
        Commands::GenerateIndex {
            property1,
            property2,
            ..
        } => generate_index(
            find_path_book(&dir_books, book_name),
            &property1,
            &property2,
        ),
        Commands::List { property, .. } => list(find_path_book(&dir_books, book_name), &property),
    };
}

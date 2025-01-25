use crate::auth::Auth;
use crate::cli_handler::{doctor_menu, get_input_string, MenuHandler};
use crate::data_structures::linked_list::LinkedList;
use crate::data_structures::map::{LocationType, Object};
use crate::data_structures::stack::Stack;
use crate::db::entities::{Ambulance, Drug, DrugGP, Patient, Prescription, Role};
use crate::data_structures::trie::Trie;


pub fn make_appointment(auth: &mut Auth) {
    let options = auth.db.clinics_data.as_ref().unwrap().iter().map(|clinic| clinic.name.as_str()).collect::<Vec<&str>>().into_iter();
    let clinic_menu = MenuHandler::new("Choose a clinic".to_string(), options);
    let selected_clinic = clinic_menu.run();
    let selected_clinic = auth.db.clinics_data.as_mut().unwrap().get_by_uniq_attr(selected_clinic).unwrap();
    let options = selected_clinic.doctors.iter().map(|doctor| doctor.as_str()).collect::<Vec<&str>>().into_iter();
    let doctor_menu = MenuHandler::new("Choose a doctor".to_string(), options);
    let selected_doctor = doctor_menu.run();

    auth.db.doctors_data.as_mut().unwrap().get_by_uniq_attr(selected_doctor).unwrap().patients.insert(Patient {
        name: auth.user.as_ref().unwrap().username.clone(),
        priority: 5 // least priority
    });

    auth.db.commit().unwrap();
}

pub fn cancel_appointment(auth: &mut Auth) {
    let options = auth.db.doctors_data.as_ref().unwrap().iter().filter_map(|doctor| {
        doctor.patients.clone().get_by_uniq_attr(auth.user.as_ref().unwrap().username.clone()).map(|_| doctor.doctor.as_str())
    }).collect::<Vec<&str>>().into_iter();
    let doctor_menu = MenuHandler::new("Choose a doctor".to_string(), options);
    let selected_doctor = doctor_menu.run();

    let selected_doctor = auth.db.doctors_data.as_mut().unwrap().get_by_uniq_attr(selected_doctor).unwrap();
    let extracted = selected_doctor.patients.remove_by_uniq_attr(auth.user.as_ref().unwrap().username.clone());
    
    if extracted {
        println!("Appointment cancelled");
    }

    auth.db.commit().unwrap();
}


pub fn visit_patients_wrapper(auth: &mut Auth) {
    loop {
        let inp = get_input_string("Enter 'done' to stop".to_string());
        let list_is_empty = auth.db.doctors_data.as_mut().unwrap().get_by_uniq_attr(auth.user.as_ref().unwrap().username.clone()).unwrap().patients.is_empty();
        if list_is_empty && inp == "done" {
            break;
        }
        visit_patients(auth);
        auth.db.commit().unwrap();
    }
    doctor_menu(auth);
}

pub fn visit_patients(auth: &mut Auth) {
    let selected_doctor = auth.db.doctors_data.as_mut().unwrap().get_by_uniq_attr(auth.user.as_ref().unwrap().username.clone()).unwrap();
    if let Some(patient) = selected_doctor.patients.pop() {
        {
            println!("Patient: {}", patient.name);
            let patient = auth.db.users_data.as_mut().unwrap().get_by_uniq_attr(patient.name.clone()).unwrap();
            println!("Patient: {}", patient.full_name);
            println!("ssn: {}", patient.ssn);
            println!("age: {}", patient.age);
        }

        let mut prescription = Stack::new();
        loop {
            let inp = get_input_string("Enter prescription based on priority (low to high) or type 'done'".to_string());
            if inp == "done" {
                break;
            }
            prescription.push(inp);
        }

        auth.db.insert_prescription(Prescription {
            patient_name: patient.name,
            medications: prescription
        }).unwrap();
    }
}

pub fn dispense_medications(auth: &mut Auth) {
    println!("Dispense medications");
    let patient_name = get_input_string("Enter patient name".to_string());
    if let Some(prescription) = auth.db.get_prescription(patient_name.clone()) {
        println!("Patient: {}", prescription.patient_name);
        println!("Medications: {:?}", prescription.medications);
        while let Some(medication) = prescription.medications.pop() {
            println!("Dispensing medication: {}", medication);
            get_input_string("".to_string());
        }
        println!("Medications dispensed");
        auth.db.remove_prescription(patient_name);
        auth.db.commit().unwrap();
    } else {
        println!("Patient not found");
    }
}

pub fn assign_patients(auth: &mut Auth) {
    let patient_username = get_input_string("Enter patient username".to_string());
    if auth.db.get_user(patient_username.clone()).is_none() {
        let patient_password = get_input_string("Enter patient password".to_string());
        let patient_full_name = get_input_string("Enter patient full name".to_string());
        let patient_ssn = get_input_string("Enter patient ssn".to_string());
        let patient_age = get_input_string("Enter patient age".to_string()).parse::<u32>().unwrap();
        auth.register(patient_username.clone(), patient_password, patient_full_name, patient_ssn, patient_age, Role::Patient).unwrap();
        auth.db.commit().unwrap();
    }

    let options = auth.db.clinics_data.as_ref().unwrap().iter().map(|clinic| clinic.name.as_str()).collect::<Vec<&str>>().into_iter();
    let clinic_menu = MenuHandler::new("Choose a clinic".to_string(), options);
    let selected_clinic = clinic_menu.run();
    let selected_clinic = auth.db.clinics_data.as_mut().unwrap().get_by_uniq_attr(selected_clinic).unwrap();
    let options = selected_clinic.doctors.iter().map(|doctor| doctor.as_str()).collect::<Vec<&str>>().into_iter();
    let doctor_menu = MenuHandler::new("Choose a doctor".to_string(), options);
    let selected_doctor = doctor_menu.run();
    let priority = get_input_string("Enter patient priority".to_string()).parse::<u32>().unwrap();

    auth.db.doctors_data.as_mut().unwrap().get_by_uniq_attr(selected_doctor).unwrap().patients.insert(Patient {
        name: patient_username,
        priority
    });

    auth.db.commit().unwrap();
}


pub fn add_drug(auth: &mut Auth) {
    let name = get_input_string("Enter drug name".to_string());
    if auth.db.get_drug_by_name(name.clone()).is_none() {
        let price = get_input_string("Enter drug price".to_string()).parse::<f32>().unwrap();
        let drug = Drug {
            id: match auth.db.drugs_data.as_ref() {
                Some(drugs) => drugs.max().id + 1,
                None => 0
                
            },
            name: name.clone(),
            price,
            quantity: 0
        };
        auth.db.insert_drug(drug).unwrap();
    }
    let quantity = get_input_string("Enter drug quantity".to_string()).parse::<u32>().unwrap();
    auth.db.get_drug_by_name(name.clone()).unwrap().quantity += quantity;
    auth.db.commit().unwrap();
    println!("Drug added");
}

pub fn remove_drug(auth: &mut Auth) {
    let id = get_input_string("Enter drug id".to_string()).parse::<u32>().unwrap();
    if let Some(drug) = auth.db.get_drug_by_id(id.clone()) {
        let quantity = get_input_string("Enter quantity to remove".to_string()).parse::<u32>().unwrap();
        if drug.quantity >= quantity {
            drug.quantity -= quantity;
            let remaining_quantity = drug.quantity;
            if remaining_quantity == 0 {
                auth.db.remove_drug(id);
            }
            auth.db.commit().unwrap();
            println!("Remained quantity: {}", remaining_quantity);
        } else {
            println!("Not enough quantity");
        }
    } else {
        println!("Drug not found");
    }
}

pub fn search_drugs(auth: &mut Auth) {
    let options = ["name", "id", "price"];
    let menu = MenuHandler::new("Search by".to_string(), options.into_iter());
    let search_type = menu.run();
    match search_type.as_str() {
        "name" => {
            let name = get_input_string("Enter drug name: ".to_string());
            if let Some(drug) = auth.db.get_drug_by_name(name.clone()) {
                println!("Drug found: {:?}", drug);
            } else {
                println!("Drug not found");

                let mut trie = Trie::new();
                for drug in auth.db.drugs_data.as_ref().unwrap().iter() {
                    trie.insert(&drug.name.to_lowercase());
                }
                let suggestions = trie.auto_complete(&name.to_lowercase());
                if suggestions.is_empty() {
                    println!("No suggestions found");
                } else {
                    println!("Suggestions: {:?}", suggestions.iter().collect::<Vec<_>>());
                }
            }
        }
        "id" => {
            let id = get_input_string("Enter drug id: ".to_string()).parse::<u32>().unwrap();
            if let Some(drug) = auth.db.get_drug_by_id(id) {
                println!("Drug found: {:?}", drug);
            } else {
                println!("Drug not found");
            }
        }
        "price" => {
            let min_price = get_input_string("Enter minimum price: ".to_string()).parse::<f32>().unwrap();
            let max_price = get_input_string("Enter maximum price: ".to_string()).parse::<f32>().unwrap();
            let drugs = auth.db.drugs_data.as_ref().unwrap().iter().filter(|drug| drug.price >= min_price && drug.price <= max_price).collect::<Vec<_>>();
            if drugs.is_empty() {
                println!("No drugs found in the given price range");
            } else {
                println!("Drugs found: {:?}", drugs);
            }
        }
        _ => {
            println!("Invalid search type");
        }
    }
}

pub fn display_all_drugs(auth: &mut Auth) {
    let drugs = auth.db.drugs_data.as_ref();
    if drugs.is_none() {
        println!("No drugs available");
        return;
    }
    let drugs = drugs.unwrap();

    let mut total_quantity = 0;
    let mut cheapest_drug = drugs.iter().next().unwrap();
    let mut most_expensive_drug = drugs.iter().next().unwrap();

    for drug in drugs.iter() {
        println!("{:?}", drug);
        total_quantity += drug.quantity;
        if drug.price < cheapest_drug.price {
            cheapest_drug = drug;
        }
        if drug.price > most_expensive_drug.price {
            most_expensive_drug = drug;
        }
    }

    println!("Total quantity of all drugs: {}", total_quantity);
    println!("Cheapest drug: {:?}", cheapest_drug);
    println!("Most expensive drug: {:?}", most_expensive_drug);
}

pub fn create_drug_gp(auth: &mut Auth) {
    let name = get_input_string("Enter drug group name".to_string());
    if auth.db.get_drug_gp(name.clone()).is_none() {
        let mut drugs = LinkedList::new();
        loop {
            let drug_name = get_input_string("Enter drug name or type 'done'".to_string());
            if drug_name == "done" {
                break;
            }
            if let Some(drug) = auth.db.get_drug_by_name(drug_name.clone()) {
                drugs.push_front(drug.id);
            } else {
                println!("Drug not found");
            }
        }
        auth.db.insert_drug_gp(DrugGP { name: name.clone(), drugs }).unwrap();
    }
    auth.db.commit().unwrap();
}

pub fn add_drug_to_gp(auth: &mut Auth) {
    let name = get_input_string("Enter drug group name".to_string());
    if let Some(drug_gp) = auth.db.get_drug_gp(name.clone()) {
        let mut drugs = drug_gp.drugs.clone();
        loop {
            let drug_name = get_input_string("Enter drug name or type 'done'".to_string());
            if drug_name == "done" {
                break;
            }
            if let Some(drug) = auth.db.get_drug_by_name(drug_name.clone()) {
                if !drugs.contains(&drug.id) {
                    drugs.push_front(drug.id);
                } else {
                    println!("Drug already exists in the group");
                }
            } else {
                println!("Drug not found");
            }
        }
        auth.db.get_drug_gp(name.clone()).unwrap().drugs = drugs;
        auth.db.commit().unwrap();
    } else {
        println!("Drug group not found");
    }
}

pub fn remove_drug_gp(auth: &mut Auth) {
    let name = get_input_string("Enter drug group name".to_string());
    if let Some(_drug_gp) = auth.db.get_drug_gp(name.clone()) {
        auth.db.remove_drug_gp(name);
        auth.db.commit().unwrap();
        println!("Drug group removed");
    } else {
        println!("Drug group not found");
    }
}

pub fn display_all_drug_gps(auth: &mut Auth) {
    let drug_gps = auth.db.drug_gps.as_ref();
    if drug_gps.is_none() {
        println!("No drug groups available");
        return;
    }
    let drug_gps = drug_gps.unwrap().clone();

    for drug_gp in drug_gps.iter() {
        println!("Drug Group: {}", drug_gp.name);
        let mut drugs = LinkedList::new();
        for id in drug_gp.drugs.iter() {
            if let Some(drug) = auth.db.get_drug_by_id(*id) {
                drugs.push_front(drug.clone());
            }
        }
        if drugs.is_empty() {
            println!("  No drugs in this group");
        } else {
            for drug in drugs.iter() {
                println!("  - {:?}", drug);
            }
        }
    }
}

pub fn show_search_complexity(auth: &mut Auth) {
    let height = auth.db.drugs_data.as_ref().unwrap().height();
    let mut result = LinkedList::new();
    auth.db.drugs_data.as_ref().unwrap().in_order_traversal_collect(&mut result);
    let total_nodes = result.len();
    println!("Total nodes in the tree: {}", total_nodes);
    println!("Height of the tree: {}", height);
    println!("Complexity of search: O(log {})", height);
}

pub fn add_location(auth: &mut Auth) {
    let name = get_input_string("Enter location name".to_string());
    if auth.db.map.nodes.get(name.as_str()).is_some() {
        println!("Location already exists, adding edges instead");
    } else {
        let options = ["Hospital", "Home", "Other"];
        let menu = MenuHandler::new("Enter location type".to_string(), options.into_iter());
        let selected = menu.run();
        let location_type = match selected.as_str() {
            "Hospital" => LocationType::Hospital,
            "Home" => LocationType::Home,
            "Other" => LocationType::Other,
            _ => {
                println!("Invalid location type");
                return;
            }
        };
        auth.db.map.add_node(name.clone(), location_type);
    }

    loop {
        let neighbor = get_input_string("Enter neighbor id or type 'done'".to_string());
        if neighbor == "done" {
            break;
        }
        if let Some(_node) = auth.db.map.nodes.get(neighbor.as_str()) {
            auth.db.map.add_edge(name.clone(), neighbor.clone());
        } else {
            println!("Neighbor not found");
        }
    }
    auth.db.commit().unwrap();
    println!("Location added");
}


pub fn remove_location(auth: &mut Auth) {
    let name: String = get_input_string("Enter location name".to_string());
    auth.db.map.remove_node(name);
    auth.db.commit().unwrap();
    println!("Location removed");
}

pub fn print_map(auth: &mut Auth) {
    auth.db.map.print_graph();
}

pub fn add_ambulance(auth: &mut Auth) {
    let name = get_input_string("Enter ambulance name".to_string());
    if auth.db.get_ambulance(name.clone()).is_some() {
        println!("Ambulance already exists");
        return;
    }
    let hospital = get_input_string("Enter hospital name".to_string());
    if auth.db.map.nodes.get(hospital.as_str()).is_none() {
        println!("Hospital not found");
        return;
    }
    let location = get_input_string("Enter the ambulance current location name".to_string());
    if auth.db.map.nodes.get(location.as_str()).is_none() {
        println!("Location not found");
        return;
    }
    
    auth.db.insert_ambulance(Ambulance::new(name.clone(), hospital.clone(), location.clone())).unwrap();
    auth.db.map.add_object_to_node(location.as_str(), Object { name });
    auth.db.commit().unwrap();
    println!("Ambulance added");
}

pub fn remove_ambulance(auth: &mut Auth) {
    let name = get_input_string("Enter ambulance name".to_string());
    let ambulance = auth.db.get_ambulance(name.clone());
    if ambulance.is_none() {
        println!("Ambulance not found");
        return;
    }
    let ambulance = ambulance.unwrap().clone();
    auth.db.map.remove_object_from_node(ambulance.location.as_str(), &name);
    auth.db.remove_ambulance(name.clone());
    auth.db.commit().unwrap();
    println!("Ambulance removed");
}

pub fn move_ambulance(auth: &mut Auth) {
    let name = get_input_string("Enter ambulance name".to_string());
    let ambulance = auth.db.get_ambulance(name.clone());
    if ambulance.is_none() {
        println!("Ambulance not found");
        return;
    }
    let ambulance = ambulance.unwrap().clone();
    let location = get_input_string("Enter new location name".to_string());
    if auth.db.map.nodes.get(location.as_str()).is_none() {
        println!("Location not found");
        return;
    }
    auth.db.ambulances_data.as_mut().unwrap().get_by_uniq_attr(name.clone()).unwrap().location = location.clone();
    auth.db.map.move_object(&ambulance.location, &location, &name).unwrap();
    auth.db.commit().unwrap();
    println!("Ambulance moved");
}

pub fn list_ambulances(auth: &mut Auth) {
    let ambulances = auth.db.ambulances_data.as_ref();
    if ambulances.is_none() {
        println!("No ambulances available");
        return;
    }
    let ambulances = ambulances.unwrap().clone();

    for ambulance in ambulances.iter() {
        println!("{:?}", ambulance);
    }
}

pub fn send_ambulance_to_patient(auth: &mut Auth) {
    let patient_loc = get_input_string("Enter patient location".to_string());
    let dst_hosp = get_input_string("Enter destination hospital".to_string());

    let mut shortest_path = None;
    let mut min_distance = std::f32::MAX;

    for ambulance in auth.db.ambulances_data.as_mut().unwrap().iter_mut() {
        let path = auth.db.map.shortest_path(&ambulance.location, &patient_loc);
        let mut distance = std::f32::MAX;
        if let Some(path) = path {
            distance = path.len() as f32;
        }
        if ambulance.hospital != dst_hosp {
            distance *= 1.2; // Penalty for ambulances from other hospitals
        }
        if distance < min_distance {
            min_distance = distance;
            shortest_path = Some((ambulance, distance));
        }
    }

    if let Some((ambulance, _)) = shortest_path {
        println!("Sending ambulance: {}", ambulance.name);
        auth.db.map.move_object(&ambulance.location, &patient_loc, &ambulance.name).unwrap();
        auth.db.map.move_object(&patient_loc, &dst_hosp, &ambulance.name).unwrap();
        println!("Ambulance sent from {} to {} via {}", ambulance.location, dst_hosp, patient_loc);
        ambulance.location = dst_hosp.clone();
        auth.db.commit().unwrap();
    } else {
        println!("No available ambulance found");
    }
}

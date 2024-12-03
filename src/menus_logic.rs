use crate::auth::Auth;
use crate::cli_handler::{doctor_menu, get_input_string, MenuHandler};
use crate::data_structures::stack::Stack;
use crate::db::entities::{Patient, Prescription, Role};


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

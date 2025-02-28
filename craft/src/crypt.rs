use std::{io::Read, path::Path};

use anyhow::{Context, Ok};
use pgp_key::{generate_key_pair,encrypt_message, decrypt_message};

use crate::pgp_key::{self, KeyPair, list_keys, delete_key};

// Default key file path 
const KEY_FILE_PATH:  &str= "../craft/key_files";

// the trait and impl for KeyPair is a preparation for crate Tongsuo. 
// a trait for key
pub trait Key {
    // type
    type PublicKey;
    type PrivateKey;

    // function
    // generate default key 
    fn generate_key();
    // generate key with primary id
    fn generate_key_full(primary_id:&str, key_name:&str);
    // encrypt with public key
    fn encrypt(public_key_file_path: &str); 
    // decrypt with private key
    fn decrypt(private_key_file_path: &str);
    // list keys
    fn list_keys(key_path: &str);
    // delete key
    fn delete_key(key_name:&str, key_path: &str);
}

// OpenPGP Key
impl Key for KeyPair {
    type PublicKey = pgp::SignedPublicKey;
    type PrivateKey = pgp::SignedSecretKey;

    fn generate_key(){
        let _ = generate_key();
    }

    fn generate_key_full(primary_id:&str, key_name:&str) {
        let _ = generate_key_full(primary_id, key_name);
    }

    fn encrypt(public_key_file_path: &str){
        let _ = encrypt_blob(public_key_file_path);
    }

    fn decrypt(private_key_file_path: &str){
        let _ = decrypt_blob(private_key_file_path);
    }

    fn list_keys(key_path: &str){
        let _ = list_keys(key_path);
    }

    fn delete_key(key_name:&str, key_path: &str){
        let _ = delete_key(key_name, key_path);
    }
}
// Generate default public key and secret key at /craft/key_files/ 
pub fn generate_key() -> Result<(), anyhow::Error>{
    println!("Creating key pair, this will take a few seconds...");
    // Create default key file path if it is not exist.
    std::fs::create_dir_all(KEY_FILE_PATH)?;
            // deafult key pair  
            let key_pair = generate_key_pair("User <craft@craft.com>").expect("Failed to generate key pair");
            // Generate a public key
            let pub_key = key_pair
                .public_key
                .to_armored_string(None)
                .expect("Failed to convert public key to armored ASCII string");
            // Write public key to pub.asc,it will replace the old pub.asc if there is already one
            _=std::fs::write( "../craft/key_files/pub.asc",pub_key).context("Writing public key to file");
            // Generate a secret key
            let sec_key = key_pair
                .secret_key
                .to_armored_string(None)
                .expect("Failed to convert secret key to armored ASCII string");
            // Write secret key to sec.asc, it will replace the old sec.asc if there is alreay one
            _=std::fs::write( "../craft/key_files/sec.asc",sec_key).context("Writing secret key to file");
            Ok(())
}

// Generate full key with pubkey, seckey, primary id.
// Arguments: primary_id, as &str, it should be written as "User <example@example.com>"; key_name, git-craft will keep ur key file as key_namepub.asc 
pub fn generate_key_full(primary_id:&str, key_name:&str)-> Result<KeyPair, anyhow::Error> {
    println!("Creating key pair, this will take a few seconds...");
    // set a key file path
    let key_file_path= Path::new("../craft/key_files");
    // Create a key file path if default path is not exist.
    std::fs::create_dir_all(key_file_path)?; 
        // generate_key_pair to generate key with a given non-default key id
        let key_pair=generate_key_pair(primary_id).expect("Failed to generate full key pair");
        // Generate a public key with primary id
        let pub_key = key_pair
            .public_key
            .to_armored_string(None)
            .expect("Failed to convert public key to armored ASCII string");
        // Add key_namepub.asc to key file path
        let pub_key_file_path =key_file_path.join(format!("{}pub.asc", key_name));
        // Write public key to file,it will replace the old same name's public key 
        _=std::fs::write( pub_key_file_path,pub_key).context("Writing public key to file");
        // Generate a secret key
        let sec_key = key_pair
            .secret_key
            .to_armored_string(None)
            .expect("Failed to convert secret key to armored ASCII string");
        // Add key_namesec.asc to key file path
        let sec_key_file_path = key_file_path.join(format!("{}sec.asc", key_name)); 
        // Write secret key to file, it will replace the old same name's secret key.
        _=std::fs::write( sec_key_file_path,sec_key).context("Writing secret key to file");    
            
    Ok(key_pair)
}
// A blob encrypt function,it can encrypt blob.data
// Argument: public_key_file_path, public key's file path; I set a default path now.  
pub fn encrypt_blob(public_key_file_path: &str)-> Result<(),anyhow::Error>{
            // Read blob data from standard input stream
            let mut blob_data = Vec::new();
            std::io::stdin().read_to_end(&mut blob_data).unwrap();
            // Get blob.data as msg to encrypt
            let msg = std::str::from_utf8(&blob_data).expect("Invalid UTF-8 sequence");
            // Encrypt the contents with the given public key 
            let encrypted = encrypt_message(msg, public_key_file_path).expect("Failed to encrypt message");
            // Print it, git will get encrypted data 
            print!("{}", &encrypted);
            Ok(())
}

// A blob decrypt function,it can decrypt blob.data encrypted by encrypted_blob()
// Arguments: secret_key_file_path; I set a default one now. 
pub fn decrypt_blob(secret_key_file_path:&str) -> Result<(),anyhow::Error>{
            // Read blob.data from standard input stream
            let mut blob_data = Vec::new();
            std::io::stdin().read_to_end(&mut blob_data).unwrap();
            // Set a encrypt_msg to get &str 
            let encrypted_msg = std::str::from_utf8(&blob_data).expect("Invalid UTF-8 sequence");
            // Decrypt contents with the given secret key
            let decrypted_msg = decrypt_message(encrypted_msg, secret_key_file_path).expect("Failed to decrypt message");
            // Print decrypted contents, then git will write decrypted contents to origin file
            print!("{}", &decrypted_msg);
            Ok(())
}

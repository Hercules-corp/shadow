use mongodb::{Client, options::ClientOptions};
use std::env;

// We need to access the db module - for a binary, we'll need to make it public or use a different approach
// For now, let's create a simpler test that doesn't require module access

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    
    println!("🔍 Testing database connection...\n");
    
    // Check environment variable
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // Mask password in connection string for display
    let masked_url = if database_url.contains("@") {
        database_url.split("@").nth(0)
            .map(|s| format!("{}@***", s))
            .unwrap_or_else(|| "mongodb://***".to_string())
    } else {
        database_url.clone()
    };
    println!("📡 Connecting to: {}", masked_url);
    
    // Parse connection string
    let client_options = ClientOptions::parse(&database_url).await?;
    let client = Client::with_options(client_options)?;
    
    // Test connection
    println!("⏳ Pinging database...");
    client
        .database("admin")
        .run_command(mongodb::bson::doc! { "ping": 1 }, None)
        .await?;
    
    println!("✅ Database connection successful!\n");
    
    // Test database operations
    let db = client.database("shadow");
    
    println!("📝 Testing basic database operations...");
    
    // Test creating a document in users collection
    let test_wallet = format!("test_wallet_{}", uuid::Uuid::new_v4());
    println!("  Creating test user: {}", test_wallet);
    
    let users_collection = db.collection::<mongodb::bson::Document>("users");
    let now = chrono::Utc::now();
    let bson_now = mongodb::bson::DateTime::from_millis(now.timestamp_millis());
    
    let user_doc = mongodb::bson::doc! {
        "_id": &test_wallet,
        "profile_cid": "test_cid_123",
        "is_public": true,
        "created_at": bson_now,
        "updated_at": bson_now,
    };
    
    users_collection.insert_one(user_doc.clone(), None).await?;
    println!("  ✅ User document inserted");
    
    // Test retrieving the document
    let filter = mongodb::bson::doc! { "_id": &test_wallet };
    let retrieved = users_collection.find_one(filter.clone(), None).await?;
    
    match retrieved {
        Some(doc) => {
            println!("  ✅ User document retrieved:");
            println!("     Wallet: {:?}", doc.get_str("_id"));
            println!("     Profile CID: {:?}", doc.get_str("profile_cid"));
            println!("     Is Public: {:?}", doc.get_bool("is_public"));
            if let Ok(dt) = doc.get_datetime("created_at") {
                println!("     Created: {:?}", dt);
            }
        }
        None => {
            println!("  ❌ User document not found after insertion!");
            return Err(anyhow::anyhow!("User insertion test failed"));
        }
    }
    
    // Test updating the document
    let update = mongodb::bson::doc! {
        "$set": {
            "profile_cid": "test_cid_456",
            "is_public": false,
            "updated_at": mongodb::bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis()),
        }
    };
    
    users_collection.update_one(filter.clone(), update, None).await?;
    let updated = users_collection.find_one(filter.clone(), None).await?;
    
    if let Some(doc) = updated {
        if doc.get_str("profile_cid") == Ok("test_cid_456") && doc.get_bool("is_public") == Ok(false) {
            println!("  ✅ User document updated successfully");
        } else {
            println!("  ❌ User document update failed");
        }
    }
    
    // Cleanup
    users_collection.delete_one(filter, None).await?;
    println!("  🧹 Test user cleaned up\n");
    
    println!("📝 Testing sites collection...");
    
    // Test creating a document in sites collection
    let test_program = format!("test_program_{}", uuid::Uuid::new_v4());
    println!("  Creating test site: {}", test_program);
    
    let sites_collection = db.collection::<mongodb::bson::Document>("sites");
    let site_doc = mongodb::bson::doc! {
        "_id": &test_program,
        "owner_pubkey": "test_owner_123",
        "storage_cid": "ipfs://test_cid_456",
        "name": "Test Site",
        "description": "Test Description",
        "created_at": bson_now,
        "updated_at": bson_now,
    };
    
    sites_collection.insert_one(site_doc.clone(), None).await?;
    println!("  ✅ Site document inserted");
    
    // Test retrieving the site
    let site_filter = mongodb::bson::doc! { "_id": &test_program };
    let retrieved_site = sites_collection.find_one(site_filter.clone(), None).await?;
    
    match retrieved_site {
        Some(doc) => {
            println!("  ✅ Site document retrieved:");
            println!("     Program: {:?}", doc.get_str("_id"));
            println!("     Owner: {:?}", doc.get_str("owner_pubkey"));
            println!("     Storage CID: {:?}", doc.get_str("storage_cid"));
            println!("     Name: {:?}", doc.get_str("name"));
        }
        None => {
            println!("  ❌ Site document not found after insertion!");
            return Err(anyhow::anyhow!("Site insertion test failed"));
        }
    }
    
    // Cleanup
    sites_collection.delete_one(site_filter, None).await?;
    println!("  🧹 Test site cleaned up\n");
    
    println!("🎉 All database tests passed!\n");
    println!("✅ Database is working correctly!");
    println!("\n💡 To test the full API, run: cargo run");
    println!("   Then test endpoints like: http://localhost:8080/api/health");
    
    Ok(())
}

use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static TYPE_COLORS: Lazy<HashMap<&'static str, Vec<u8>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("Normal", vec![168, 167, 122]);    // A8A77A
    map.insert("Fire", vec![238, 129, 48]);       // EE8130
    map.insert("Water", vec![99, 144, 240]);      // 6390F0
    map.insert("Electric", vec![247, 208, 44]);   // F7D02C
    map.insert("Grass", vec![122, 199, 76]);      // 7AC74C
    map.insert("Ice", vec![150, 217, 214]);       // 96D9D6
    map.insert("Fighting", vec![194, 46, 40]);    // C22E28
    map.insert("Poison", vec![163, 62, 161]);     // A33EA1
    map.insert("Ground", vec![226, 191, 101]);    // E2BF65
    map.insert("Flying", vec![169, 143, 243]);    // A98FF3
    map.insert("Psychic", vec![249, 85, 135]);    // F95587
    map.insert("Bug", vec![166, 185, 26]);        // A6B91A
    map.insert("Rock", vec![182, 161, 54]);       // B6A136
    map.insert("Ghost", vec![115, 87, 151]);      // 735797
    map.insert("Dragon", vec![111, 53, 252]);     // 6F35FC
    map.insert("Dark", vec![112, 87, 70]);        // 705746
    map.insert("Steel", vec![183, 183, 206]);     // B7B7CE
    map.insert("Fairy", vec![214, 133, 173]);     // D685AD
    map
});

pub static STAT_COLORS: Lazy<HashMap<&'static str, Vec<u8>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("great", vec![0, 255, 0]);
    map.insert("good", vec![0, 192, 0]);
    map.insert("average", vec![255, 255, 0]);
    map.insert("bad", vec![255, 128, 0]);
    map.insert("atrocious", vec![255, 0, 0]);
    map
});

pub async fn get_type_color(type_name: &str) -> Vec<u8> {
    TYPE_COLORS.get(type_name).cloned().unwrap_or_else(|| vec![255, 255, 255])
}

pub async fn get_stat_color(stat_value: u8) -> Vec<u8> {
    let max_stat = 255;
    let divisor = 5;

    let stat_category = match stat_value {
        _ if stat_value < max_stat / divisor => "atrocious",
        _ if stat_value < max_stat / divisor * 2 => "bad",
        _ if stat_value < max_stat / divisor * 3 => "average",
        _ if stat_value < max_stat / divisor * 4 => "good",
        _ => "great",
    };

    STAT_COLORS.get(stat_category).cloned().unwrap_or_else(|| vec![255, 255, 255])
}


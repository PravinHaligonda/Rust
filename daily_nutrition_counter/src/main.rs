fn main() {
    println!("Hello, Pravin!");
    daily_plan_with_time();

    println!("Tips for Timing: ");
    tips_for_timing();

    println!("Hydration: ");
    hydration();

    println!("Macro Nutrients Breakdown: ");
    macro_nutrient_breakdown();

    println!("Micro Nutrients Breakdown: ");
    micro_nutrients();

    println!("Fiber: ");
    fiber();

    println!("Base Metabolic Rate (BMR): ");
    base_meta_rate();

    println!("Total Daily Energy Expenditure (TDEE): ");
    tdee();
}

fn base_meta_rate() {
    println!("Approx. 1300-1400 kcal/day");
}

fn tdee() {
    println!("Total Daily Energy Expenditure. (TDEE)");
    println!("Sedentary: ~1600-1700 kcal/day");
    println!("Moderately Active: ~1900-2000 kcal/day");
    println!("Active: ~2200+ kcal/day");
    println!("Aim for +200-300 kcal/day to gain weight gain gradually");
}

fn macro_nutrient_breakdown() {
    println!("Proteins: ");
    println!("~1.2-1.6 g/kg of body weight (maintain or gain weight)");
    println!("58-78 g/day (~20-30% of total calories)");
    println!("Sources: Standby");

    println!("Carbohydrates: ");
    println!("~4-5 g/kg of body weight (maintain or gain weight)");
    println!("200-250 g/day");
    println!("Sources: Roti, Rice, Vegetables, Whole grains");

    println!("Fats: ");
    println!("~0.8-1 g/kg of body weight (maintain or gain weight)");
    println!("40-50 g/day");
    println!("Sources: Standby");
}

fn fiber() {
    println!("Recommeded intake: 25-30 g/day");
    println!("Sources: Whole grains, Vegetables, fruits, nuts, beans, seeds");
}

fn micro_nutrients() {
    println!("Ensure you have these daily needs");
    
    println!("Calcium: ");
    println!("1000-2000 mg");
    println!("Sources: Milk, yogurts, leafy greens");

    println!("Iron: ");
    println!("~18 mg");
    println!("Sources: lentils, spinach, fortified cereals");

    println!("Vitamin D: ");
    println!("600-800 IU");
    println!("Sources: Sunlight");

    println!("Vitamin C: ");
    println!("75-90 mg");
    println!("Sources: Tomato");

    println!("Potassium: ");
    println!("~3500-4700 mg");
    println!("Sources: Potato, Banana");
}

fn hydration() {
    println!("Drink atleast 2-3 litre of water daily");
}

fn daily_plan_with_time() {
    
    println!("Breakfast: ");
    println!("7:00-8:00 AM");
    println!("2 Boiled Potatos, 1 Bowl of Moth beans, 1 Cup of Milk, Handful of Socked groundnuts");

    println!("Mid Morning Snacks: ");
    println!("10:00 - 11:00 AM");
    println!("Handful of nuts(almonds/walnuts), 1 Boiled Egg");

    println!("Lunch: ");
    println!("1:00-2:00 PM");
    println!("1-2 Rotis, 1 Bowl Rice, 1 cup lentils(dal), 1 bowl cooked vegetables, and salad");

    println!("Evening Snack: ");
    println!("4:00-5:00 PM");
    println!("Yogurt with Seeds(Chia/flux) and Fruits");

    println!("Dinner: ");
    println!("7:30-8:30 PM");
    println!("1–2 rotis, 1 bowl rice, 1 cup lentils (dal), 1 bowl cooked vegetables, with an additional egg");

    println!("Before Bed: ");
    println!("9:30-10:00 PM");
    println!("1 Glass of milk with nuts or seeds");
}

fn tips_for_timing() {
    println!("Maintain a 2-3 hour gap between meals/snacks.");
    println!("Avoid eating a heavy meal right before bed");
    println!("dinner should ideally be completed 2-3 hours before sleeping.");
    println!("Stay hydrated throughout the day by drinking water between meals.");
}
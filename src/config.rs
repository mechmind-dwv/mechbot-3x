// Inspección automatizada de calidad
let mut inspector = Robot::new_inspector(config)?;
inspector.scan_area(area_bounds).await?;
let defects = inspector.detect_defects().await?;

### **2. 🏠 Robótica Doméstica**
// Navegación autónoma en interiores
let mut home_bot = Robot::new_home_assistant(config)?;
home_bot.map_environment().await?;
home_bot.navigate_to_room("kitchen").await?;

### **3. 🚧 Exploración y Mapeo**
// SLAM (Simultaneous Localization and Mapping)
let mut explorer = Robot::new_explorer(config)?;
explorer.start_slam().await?;
let map = explorer.get_current_map().await?;

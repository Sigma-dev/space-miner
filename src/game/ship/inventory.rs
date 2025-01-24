use bevy::prelude::*;

#[derive(Component)]
pub struct Inventory {
    amount: u32,
    max_amount: u32,
}

impl Inventory {
    pub fn new(max_amount: u32) -> Inventory {
        Inventory {
            amount: 0,
            max_amount,
        }
    }

    pub fn can_add(&self, amount: u32) -> bool {
        if self.amount + amount > self.max_amount {
            return false;
        }
        true
    }

    pub fn try_add(&mut self, amount: u32) -> bool {
        if !self.can_add(amount) {
            return false;
        }
        self.amount += amount;
        true
    }
}

#[derive(Event)]
pub struct InventoryUpdate {
    pub new_amount: u32,
    pub max_amount: u32,
}

pub fn inventory_plugin(app: &mut App) {
    app.add_event::<InventoryUpdate>()
        .add_systems(Update, handle_changes);
}

fn handle_changes(
    mut events: EventWriter<InventoryUpdate>,
    inventory_q: Query<&Inventory, Changed<Inventory>>,
) {
    for inventory in inventory_q.iter() {
        events.send(InventoryUpdate {
            new_amount: inventory.amount,
            max_amount: inventory.max_amount,
        });
    }
}

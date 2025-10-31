use snake_game::{
    rng::Seeded,
    settings::{Settings, SettingsError, SettingsStore},
    state::GameState,
    types::GridSize,
};

#[test]
fn settings_default_is_valid() {
    let s = Settings::default();
    assert!(s.validate().is_ok());
}

#[test]
fn settings_invalid_grid_sizes_are_rejected() {
    let bad_w = Settings::new(GridSize { w: 0, h: 10 }, 10);
    assert!(matches!(bad_w, Err(SettingsError::InvalidGridWidth(0))));

    let bad_h = Settings::new(GridSize { w: 10, h: -1 }, 10);
    assert!(matches!(bad_h, Err(SettingsError::InvalidGridHeight(-1))));
}

#[test]
fn settings_invalid_speed_is_rejected() {
    let too_slow = Settings::new(GridSize { w: 10, h: 10 }, 0);
    assert!(matches!(too_slow, Err(SettingsError::InvalidSpeed(0))));

    let too_fast = Settings::new(GridSize { w: 10, h: 10 }, 1000);
    assert!(matches!(too_fast, Err(SettingsError::InvalidSpeed(1000))));
}

#[test]
fn settings_with_grid_and_speed_chain_validates() {
    let s = Settings::default()
        .with_grid(GridSize { w: 12, h: 8 }).unwrap()
        .with_speed(30).unwrap();
    assert_eq!(s.grid, GridSize { w: 12, h: 8 });
    assert_eq!(s.speed, 30);
}

#[test]
fn apply_to_new_game_uses_settings_grid() {
    let s = Settings::new(GridSize { w: 20, h: 15 }, 15).unwrap();
    let g: GameState = s.apply_to_new_game(Seeded::new(42));
    assert_eq!(g.grid, GridSize { w: 20, h: 15 });
}

#[test]
fn settings_store_persists_updates_in_memory() {
    let mut store = SettingsStore::default();
    let mut s = store.get();
    assert_eq!(s.grid, GridSize { w: 10, h: 10 });
    assert_eq!(s.speed, 10);

    s = s.with_grid(GridSize { w: 16, h: 12 }).unwrap().with_speed(25).unwrap();
    store.update(s).unwrap();

    let after = store.get();
    assert_eq!(after.grid, GridSize { w: 16, h: 12 });
    assert_eq!(after.speed, 25);
}



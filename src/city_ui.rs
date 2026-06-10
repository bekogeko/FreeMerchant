use crate::city::City;
use crate::map::{CITY_SIZE, MapPosition};
use crate::road::Road;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Default, Resource)]
struct SelectedCity(Option<Entity>);

#[derive(Component)]
struct CityDetailsPanel;

#[derive(Component)]
struct CityNameText;

#[derive(Component)]
struct CityRoadsText;

pub struct CityUIPlugin;

impl Plugin for CityUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedCity>()
            .add_systems(Startup, spawn_city_details_panel)
            .add_systems(Update, (select_city_on_click, update_city_details).chain());
    }
}

fn spawn_city_details_panel(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(24.0),
            right: Val::Px(24.0),
            width: Val::Px(280.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            padding: UiRect::all(Val::Px(16.0)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.06, 0.07, 0.88)),
        Visibility::Hidden,
        CityDetailsPanel,
        children![
            (
                Text::new("City"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.60, 0.76, 0.86)),
            ),
            (
                Text::new(""),
                TextFont {
                    font_size: 26.0,
                    ..default()
                },
                TextColor(Color::srgb(0.96, 0.94, 0.88)),
                CityNameText,
            ),
            (
                Text::new(""),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.82, 0.82, 0.76)),
                CityRoadsText,
            ),
        ],
    ));
}

fn select_city_on_click(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    cities: Query<(Entity, &MapPosition), With<City>>,
    ui_nodes: Query<(
        &ComputedNode,
        &UiGlobalTransform,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
    )>,
    mut selected_city: ResMut<SelectedCity>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_position) = primary_window.cursor_position() else {
        return;
    };
    let Some(physical_cursor_position) = primary_window.physical_cursor_position() else {
        return;
    };

    if cursor_over_visible_ui_node(physical_cursor_position, &ui_nodes) {
        return;
    }

    let (camera, camera_transform) = *camera;
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    selected_city.0 = cities
        .iter()
        .filter_map(|(entity, MapPosition(position))| {
            let distance = world_position.distance(*position);
            (distance <= CITY_SIZE / 2.0).then_some((entity, distance))
        })
        .min_by(|(_, left), (_, right)| left.total_cmp(right))
        .map(|(entity, _)| entity);
}

fn cursor_over_visible_ui_node(
    cursor_position: Vec2,
    ui_nodes: &Query<(
        &ComputedNode,
        &UiGlobalTransform,
        Option<&Visibility>,
        Option<&InheritedVisibility>,
    )>,
) -> bool {
    ui_nodes
        .iter()
        .any(|(node, transform, visibility, inherited_visibility)| {
            ui_node_is_visible(visibility, inherited_visibility)
                && node.contains_point(*transform, cursor_position)
        })
}

fn ui_node_is_visible(
    visibility: Option<&Visibility>,
    inherited_visibility: Option<&InheritedVisibility>,
) -> bool {
    match visibility {
        Some(Visibility::Hidden) => false,
        Some(Visibility::Visible) => true,
        Some(Visibility::Inherited) | None => {
            inherited_visibility.is_none_or(|visibility| visibility.get())
        }
    }
}

fn update_city_details(
    selected_city: Res<SelectedCity>,
    mut panel_visibility: Single<&mut Visibility, With<CityDetailsPanel>>,
    mut name_text: Single<&mut Text, (With<CityNameText>, Without<CityRoadsText>)>,
    mut roads_text: Single<&mut Text, (With<CityRoadsText>, Without<CityNameText>)>,
    cities: Query<&City>,
    roads: Query<&Road>,
) {
    let Some(selected_city) = selected_city.0 else {
        **panel_visibility = Visibility::Hidden;
        return;
    };

    let Ok(city) = cities.get(selected_city) else {
        **panel_visibility = Visibility::Hidden;
        return;
    };

    **panel_visibility = Visibility::Visible;
    name_text.0.clone_from(&city.name);
    roads_text.0 = road_list_text(selected_city, &roads, &cities);
}

fn road_list_text(selected_city: Entity, roads: &Query<&Road>, cities: &Query<&City>) -> String {
    let connected_cities = roads.iter().filter_map(|road| {
        if road.city_a == selected_city {
            cities.get(road.city_b).ok()
        } else if road.city_b == selected_city {
            cities.get(road.city_a).ok()
        } else {
            None
        }
    });

    let mut text = String::from("Roads");
    let mut has_roads = false;

    for city in connected_cities {
        has_roads = true;
        text.push_str("\n- ");
        text.push_str(&city.name);
    }

    if !has_roads {
        text.push_str("\nNo roads yet");
    }

    text
}

#[cfg(test)]
mod tests {
    use bevy::{
        camera::{RenderTargetInfo, Viewport},
        prelude::*,
        window::{PrimaryWindow, WindowResolution},
    };

    use super::*;

    const WINDOW_SIZE: Vec2 = Vec2::new(1280.0, 720.0);
    const UI_PHYSICAL_CURSOR_POSITION: Vec2 = Vec2::new(920.0, 120.0);

    #[test]
    fn click_on_visible_ui_node_keeps_selected_city() {
        let mut app = selection_test_app(UI_PHYSICAL_CURSOR_POSITION);
        let selected = spawn_selected_city(&mut app);
        spawn_ui_node_at_cursor(
            &mut app,
            UI_PHYSICAL_CURSOR_POSITION,
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
        );

        click_left_mouse(&mut app);

        assert_eq!(app.world().resource::<SelectedCity>().0, Some(selected));
    }

    #[test]
    fn click_on_hidden_ui_node_can_still_deselect_selected_city() {
        let mut app = selection_test_app(UI_PHYSICAL_CURSOR_POSITION);
        spawn_selected_city(&mut app);
        spawn_ui_node_at_cursor(
            &mut app,
            UI_PHYSICAL_CURSOR_POSITION,
            Visibility::Hidden,
            InheritedVisibility::HIDDEN,
        );

        click_left_mouse(&mut app);

        assert_eq!(app.world().resource::<SelectedCity>().0, None);
    }

    #[test]
    fn click_on_visible_ui_node_uses_physical_cursor_position() {
        let mut app = selection_test_app_with_scale(UI_PHYSICAL_CURSOR_POSITION / 2.0, 2.0);
        let selected = spawn_selected_city(&mut app);
        spawn_ui_node_at_cursor(
            &mut app,
            UI_PHYSICAL_CURSOR_POSITION,
            Visibility::Visible,
            InheritedVisibility::VISIBLE,
        );

        click_left_mouse(&mut app);

        assert_eq!(app.world().resource::<SelectedCity>().0, Some(selected));
    }

    fn selection_test_app(cursor_position: Vec2) -> App {
        selection_test_app_with_scale(cursor_position, 1.0)
    }

    fn selection_test_app_with_scale(cursor_position: Vec2, scale_factor: f32) -> App {
        let mut app = App::new();
        app.init_resource::<ButtonInput<MouseButton>>()
            .init_resource::<SelectedCity>()
            .add_systems(Update, select_city_on_click);

        let mut window = Window {
            resolution: WindowResolution::new(WINDOW_SIZE.x as u32, WINDOW_SIZE.y as u32)
                .with_scale_factor_override(scale_factor),
            ..default()
        };
        window.set_cursor_position(Some(cursor_position));
        app.world_mut().spawn((window, PrimaryWindow));
        app.world_mut()
            .spawn((test_camera(scale_factor), GlobalTransform::default()));

        app
    }

    fn spawn_selected_city(app: &mut App) -> Entity {
        let selected = app
            .world_mut()
            .spawn((City::new("Amsterdam"), MapPosition(Vec2::new(-260.0, 80.0))))
            .id();

        app.world_mut().resource_mut::<SelectedCity>().0 = Some(selected);

        selected
    }

    fn spawn_ui_node_at_cursor(
        app: &mut App,
        cursor_position: Vec2,
        visibility: Visibility,
        inherited_visibility: InheritedVisibility,
    ) {
        app.world_mut().spawn((
            Node::default(),
            ComputedNode {
                size: Vec2::new(280.0, 180.0),
                ..default()
            },
            UiGlobalTransform::from_translation(cursor_position),
            visibility,
            inherited_visibility,
        ));
    }

    fn click_left_mouse(app: &mut App) {
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);
        app.update();
    }

    fn test_camera(scale_factor: f32) -> Camera {
        let viewport = Viewport {
            physical_size: WINDOW_SIZE.as_uvec2(),
            ..default()
        };
        let mut projection = Projection::Orthographic(OrthographicProjection::default_2d());
        projection.update(WINDOW_SIZE.x, WINDOW_SIZE.y);

        let mut camera = Camera {
            viewport: Some(viewport.clone()),
            ..default()
        };
        camera.computed.target_info = Some(RenderTargetInfo {
            physical_size: viewport.physical_size,
            scale_factor,
        });
        camera.computed.clip_from_view = projection.get_clip_from_view();
        camera
    }
}

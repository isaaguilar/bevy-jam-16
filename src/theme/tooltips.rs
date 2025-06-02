use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, show_tooltips);
}

#[derive(Component)]
pub struct TooltipParent;

#[derive(Component)]
pub struct Tooltip;

pub fn show_tooltips(
    mut tooltip_parents: Query<
        (&Interaction, &Children),
        (Changed<Interaction>, With<TooltipParent>),
    >,
    mut tooltips: Query<&mut Visibility, With<Tooltip>>,
) {
    for (interaction, children) in &mut tooltip_parents {
        for child in children.iter() {
            if let Ok(mut visibility) = tooltips.get_mut(child) {
                match interaction {
                    Interaction::Hovered => {
                        *visibility = Visibility::Inherited;
                    }
                    _ => {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

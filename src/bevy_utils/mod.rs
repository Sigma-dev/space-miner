use bevy::ecs::*;
use entity::*;
use query::*;
use system::Query;

pub fn query_double<'a, D, E, F, G>(
    q1: &'a Query<D, F>,
    q2: &'a Query<E, G>,
    e1: Entity,
    e2: Entity,
) -> Option<(QueryItem<'a, D>, QueryItem<'a, E>)>
where
    D: QueryData + 'a,
    E: QueryData + 'a,
    <D as QueryData>::ReadOnly: WorldQuery<Item<'a> = QueryItem<'a, D>>,
    <E as QueryData>::ReadOnly: WorldQuery<Item<'a> = QueryItem<'a, E>>,
    F: QueryFilter,
    G: QueryFilter,
{
    if q1.contains(e1) && q2.contains(e2) {
        return Some((q1.get(e1).unwrap(), q2.get(e2).unwrap()));
    }
    if q1.contains(e2) && q2.contains(e1) {
        return Some((q1.get(e2).unwrap(), q2.get(e1).unwrap()));
    }
    None
}

pub fn query_double_mut<'a, D, E, F, G>(
    q1: &'a mut Query<D, F>,
    q2: &'a mut Query<E, G>,
    e1: Entity,
    e2: Entity,
) -> Option<(QueryItem<'a, D>, QueryItem<'a, E>)>
where
    D: QueryData,
    E: QueryData,
    F: QueryFilter,
    G: QueryFilter,
{
    if q1.contains(e1) && q2.contains(e2) {
        return Some((q1.get_mut(e1).unwrap(), q2.get_mut(e2).unwrap()));
    }
    if q1.contains(e2) && q2.contains(e1) {
        return Some((q1.get_mut(e2).unwrap(), q2.get_mut(e1).unwrap()));
    }
    None
}

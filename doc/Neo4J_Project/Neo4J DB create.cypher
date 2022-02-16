CREATE DATABASE taskscore IF NOT EXISTS;

CREATE OR REPLACE ROLE taskscore_admin_role;

CREATE OR REPLACE USER tsadmin
	SET PASSWORD 'G3n3ricPwd' CHANGE NOT REQUIRED
	SET STATUS ACTIVE
	SET HOME DATABASE taskscore;
	
GRANT ROLE taskscore_admin_role TO tsadmin;

GRANT ALL ON HOME DATABASE TO taskscore_admin_role;

:USE taskscore;

CREATE (u_fl:Person {username: 'roterkohl', display_name: 'Flori', password: 'Flori1234', is_admin: true }),
	(u_mi:Person {username: 'brutours.de', display_name: 'Michi', password: 'Michi1234' }),
	(u_fr:Person {username: 'dliwespf', display_name: 'Franki', password: 'Franki1234' }),
	(u_to:Person {username: 'topher', display_name: 'Topher', password: 'Topheri1234', is_admin: true }),

	(t_bl:Task { id: 1, name: 'Blumen gießen', points: 10, enabled: true }),
	(t_st:Task { id: 2, name: 'Stunden abgeben', points: 30, enabled: false }),
	(t_sp:Task { id: 3, name: 'Spülmaschine ausräumen', points: 52, enabled: true }),
	(t_ka:Task { id: 4, name: 'Kaffee kochen', points: 75, enabled: true }),
	
	(te_ba: Team { id: 1, name: 'Babes' }),
	(te_ch: Team { id: 2, name: 'Church' }),
	(u_fl)-[:MANAGES] -> (te_ba),
	(u_mi)-[:MANAGES] -> (te_ch),
	
	(u_fl)-[:MEMBER_OF] -> (te_ba),
	(u_mi)-[:MEMBER_OF] -> (te_ch),
	(u_fr)-[:MEMBER_OF] -> (te_ch),
	(u_fl)-[:MEMBER_OF] -> (te_ch),
	(u_to)-[:MEMBER_OF] -> (te_ch),
	
	(u_fl)-[:SCORED {points: 10, scored_at: localdatetime()}] -> (t_bl),
	(u_fl)-[:SCORED {points: 10, scored_at: localdatetime()}] -> (t_bl),
	(u_fl)-[:SCORED {points: 30, scored_at: localdatetime()}] -> (t_st),
	(u_fl)-[:SCORED {points: 10, scored_at: localdatetime()}] -> (t_bl),
	(u_fl)-[:SCORED {points: 75, scored_at: localdatetime()}] -> (t_ka),
	(u_fl)-[:SCORED {points: 52, scored_at: localdatetime()}] -> (t_sp),
	
	(u_mi)-[:SCORED {points: 10, scored_at: localdatetime()}] -> (t_bl),
	(u_mi)-[:SCORED {points: 30, scored_at: localdatetime()}] -> (t_st),
	(u_mi)-[:SCORED {points: 52, scored_at: localdatetime()}] -> (t_sp),
	(u_mi)-[:SCORED {points: 75, scored_at: localdatetime()}] -> (t_ka),
	
	(u_to)-[:SCORED {points: 75, scored_at: localdatetime()}] -> (t_ka),
	(u_to)-[:SCORED {points: 75, scored_at: localdatetime()}] -> (t_ka),
	(u_to)-[:SCORED {points: 75, scored_at: localdatetime()}] -> (t_ka),
	(u_to)-[:SCORED {points: 75, scored_at: localdatetime()}] -> (t_ka),
	(u_to)-[:SCORED {points: 75, scored_at: localdatetime()}] -> (t_ka)
	

-- Migration: refresh lowongan deadlines to plausible future dates.
-- Original seed dates (Mar-May 2026) have already passed; this UPDATE
-- pushes them to Jul-Sep 2026 so the UI does not show "Tenggat berakhir".
-- Idempotent: re-running the migration applies the same target dates.

UPDATE lowongan SET deadline = '2026-07-31' WHERE id = '33333333-0000-4000-8000-000000000001';
UPDATE lowongan SET deadline = '2026-08-15' WHERE id = '33333333-0000-4000-8000-000000000002';
UPDATE lowongan SET deadline = '2026-08-30' WHERE id = '33333333-0000-4000-8000-000000000003';
UPDATE lowongan SET deadline = '2026-09-15' WHERE id = '33333333-0000-4000-8000-000000000004';
UPDATE lowongan SET deadline = '2026-08-10' WHERE id = '33333333-0000-4000-8000-000000000005';
UPDATE lowongan SET deadline = '2026-07-25' WHERE id = '33333333-0000-4000-8000-000000000006';

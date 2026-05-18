/**
 * Barrel export for admin-shell shared components. Consumers should import
 * from `$lib/admin/components` rather than reaching into individual files so
 * future refactors (renames, splits) stay encapsulated in this directory.
 */
export { default as DataTable } from './DataTable.svelte';
export { default as FormField } from './FormField.svelte';
export { default as DeleteConfirm } from './DeleteConfirm.svelte';
export { default as SearchInput } from './SearchInput.svelte';

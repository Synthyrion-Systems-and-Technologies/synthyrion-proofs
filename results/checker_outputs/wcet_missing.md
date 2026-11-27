# synthyrion_wcetan missing (host level; provisional costs only to see past the first refusal, NO bound is claimed; lists the rows it can reach: `check` is authoritative)

| entry | status | functions reached | callee rows to write (DISTINCT callees) | callee lines listed below (one per callee AND calling function) | loop rows to write |
|---|---|---:|---:|---:|---:|
| syscall_dispatch | PENDING | 566 | 71 | 123 | 43 |
| retype | PENDING | 448 | 36 | 45 | 21 |
| revoke_untyped_step | PENDING | 820 | 81 | 118 | 73 |
| map_frame | PENDING | 284 | 24 | 38 | 0 |
| cnode_delete | PENDING | 807 | 75 | 105 | 69 |
| cnode_revoke_step | PENDING | 807 | 76 | 106 | 70 |
| ring_Ring__enter | PENDING | 10 | 1 | 1 | 0 |
| selfheal_Kernel__selfheal_step | PENDING | 692 | 59 | 75 | 23 |
| ept_map_frame | PENDING | 171 | 1 | 2 | 0 |

**9 PENDING entries; across every entry's reachable code: 186 distinct callee rows and 128 loop rows missing.**

## every function with a gap (crates the entries name)

callee `<V as Clone>::clone` in `synthyrion_btab::<Full as Clone>::clone`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_btab::<Full as Debug>::fmt`
callee `<V as PartialEq>::eq` in `synthyrion_btab::<Full as PartialEq>::eq`
callee `Formatter::write_str` in `synthyrion_capauth::<CapAuthority as core::fmt::Debug>::fmt`
callee `DebugStruct::field` in `synthyrion_capauth::<SealedCap as core::fmt::Debug>::fmt`
callee `DebugStruct::finish` in `synthyrion_capauth::<SealedCap as core::fmt::Debug>::fmt`
callee `Formatter::debug_struct` in `synthyrion_capauth::<SealedCap as core::fmt::Debug>::fmt`
callee `BTreeMap::get` in `synthyrion_capauth::CapAuthority::epoch_of`
callee `BTreeMap::entry` in `synthyrion_capauth::CapAuthority::revoke_object`
callee `alloc::collections::btree_map::Entry::or_insert` in `synthyrion_capauth::CapAuthority::revoke_object`
callee `BTreeMap::new` in `synthyrion_capauth::CapAuthority::with_secret_inner`
callee `EmbeddedMonitor::new` in `synthyrion_capauth::CapAuthority::with_secret_inner`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_gatecases::<A as Debug>::fmt`
callee `Formatter::write_str` in `synthyrion_gatecases::<Act as Debug>::fmt`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_gatecases::<Facts as Debug>::fmt`
callee `<u32 as Default>::default` in `synthyrion_gatecases::<NullIommu as Default>::default`
callee `<String as Clone>::clone` in `synthyrion_gatecases::<Outcome as Clone>::clone`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_gatecases::<Outcome as Debug>::fmt`
callee `Formatter::debug_tuple_field2_finish` in `synthyrion_gatecases::<Outcome as Debug>::fmt`
callee `Formatter::write_str` in `synthyrion_gatecases::<Outcome as Debug>::fmt`
callee `<&String as PartialEq>::eq` in `synthyrion_gatecases::<Outcome as PartialEq>::eq`
callee `<&usize as PartialEq>::eq` in `synthyrion_gatecases::<Outcome as PartialEq>::eq`
callee `<Option<(u64, Vec<u64>)> as Clone>::clone` in `synthyrion_gatecases::<Step as Clone>::clone`
callee `<Option<Act> as Clone>::clone` in `synthyrion_gatecases::<Step as Clone>::clone`
callee `<Option<After> as Clone>::clone` in `synthyrion_gatecases::<Step as Clone>::clone`
callee `<Option<u64> as Clone>::clone` in `synthyrion_gatecases::<Step as Clone>::clone`
callee `<String as Clone>::clone` in `synthyrion_gatecases::<Step as Clone>::clone`
callee `<Vec<(u64, Vec<u64>)> as Clone>::clone` in `synthyrion_gatecases::<Step as Clone>::clone`
callee `<[A; 4] as Clone>::clone` in `synthyrion_gatecases::<Step as Clone>::clone`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_gatecases::<Step as Debug>::fmt`
callee `BTreeSet::insert` in `synthyrion_gatecases::<VaultDouble as microkernel::VaultService>::grant`
callee `<Zip<core::slice::IterMut<'_, u8>, core::slice::Iter<'_, u8>> as IntoIterator>::into_iter` in `synthyrion_gatecases::<VaultDouble as microkernel::VaultService>::operate`
callee `<Zip<core::slice::IterMut<'_, u8>, core::slice::Iter<'_, u8>> as Iterator>::next` in `synthyrion_gatecases::<VaultDouble as microkernel::VaultService>::operate`
callee `<core::slice::IterMut<'_, u8> as Iterator>::zip` in `synthyrion_gatecases::<VaultDouble as microkernel::VaultService>::operate`
callee `BTreeSet::contains` in `synthyrion_gatecases::<VaultDouble as microkernel::VaultService>::operate`
loop `synthyrion_gatecases::<VaultDouble as microkernel::VaultService>::operate` header bb8
callee `BTreeSet::clear` in `synthyrion_gatecases::<VaultDouble as microkernel::VaultService>::revoke_all`
callee `Formatter::write_str` in `synthyrion_gatecases::<Who as Debug>::fmt`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_gatecases::<WorldError as Debug>::fmt`
callee `<&str as PartialEq>::eq` in `synthyrion_gatecases::<WorldError as PartialEq>::eq`
callee `Arguments::new` in `synthyrion_gatecases::<WorldError as core::fmt::Display>::fmt`
callee `Formatter::write_fmt` in `synthyrion_gatecases::<WorldError as core::fmt::Display>::fmt`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_gatecases::<WorldError as core::fmt::Display>::fmt`
callee `Vec::push` in `synthyrion_gatecases::Step::mem`
callee `alloc::slice::to_vec` in `synthyrion_gatecases::Step::mem`
callee `alloc::slice::to_vec` in `synthyrion_gatecases::Step::out`
callee `<str as ToString>::to_string` in `synthyrion_gatecases::Step::root`
callee `Vec::new` in `synthyrion_gatecases::Step::root`
callee `microkernel::cspace::cs_lookup` in `synthyrion_gatecases::act`
callee `microkernel::ipcx::raise_fault` in `synthyrion_gatecases::act`
callee `microkernel::vcpu::rv::rvcpu_enter` in `synthyrion_gatecases::act`
callee `microkernel::vcpu::rv::rvcpu_exited` in `synthyrion_gatecases::act`
callee `microkernel::vcpu::vcpu_enter` in `synthyrion_gatecases::act`
callee `microkernel::vcpu::vcpu_exited` in `synthyrion_gatecases::act`
callee `<Filter<Zip<core::slice::Iter<'_, SealedCap>, core::slice::Iter<'_, UntypedDesc>>, {closure@crates\synthyrion_gatecases\src\lib.rs:201:72: 201:80}> as Iterator>::map` in `synthyrion_gatecases::build_world`
callee `<Option<SealedCap> as Need<SealedCap>>::need` in `synthyrion_gatecases::build_world`
callee `<Option<SlotView> as Need<SlotView>>::need` in `synthyrion_gatecases::build_world`
callee `<Result<(), CsError> as Need<()>>::need` in `synthyrion_gatecases::build_world`
callee `<Result<(), CsError> as PartialEq>::ne` in `synthyrion_gatecases::build_world`
callee `<Result<(), IoError> as PartialEq>::ne` in `synthyrion_gatecases::build_world`
callee `<Result<(), WorldError> as Try>::branch` in `synthyrion_gatecases::build_world`
callee `<Result<Facts, WorldError> as FromResidual<Result<Infallible, WorldError>>>::from_residual` in `synthyrion_gatecases::build_world`
callee `<Result<RootTask, BootError> as Need<RootTask>>::need` in `synthyrion_gatecases::build_world`
callee `<Result<RootTask, WorldError> as Try>::branch` in `synthyrion_gatecases::build_world`
callee `<Result<SealedCap, HvError> as Need<SealedCap>>::need` in `synthyrion_gatecases::build_world`
callee `<Result<SealedCap, ObjError> as Need<SealedCap>>::need` in `synthyrion_gatecases::build_world`
callee `<Result<SealedCap, WorldError> as Try>::branch` in `synthyrion_gatecases::build_world`
callee `<Result<SlotView, WorldError> as Try>::branch` in `synthyrion_gatecases::build_world`
callee `<Result<Vec<SealedCap>, ObjError> as Need<Vec<SealedCap>>>::need` in `synthyrion_gatecases::build_world`
callee `<Result<Vec<SealedCap>, WorldError> as Try>::branch` in `synthyrion_gatecases::build_world`
callee `<Vec<SealedCap> as Deref>::deref` in `synthyrion_gatecases::build_world`
callee `<Vec<SealedCap> as IntoIterator>::into_iter` in `synthyrion_gatecases::build_world`
callee `<Zip<core::slice::Iter<'_, SealedCap>, core::slice::Iter<'_, UntypedDesc>> as Iterator>::filter` in `synthyrion_gatecases::build_world`
callee `<[u8; 3] as IntoIterator>::into_iter` in `synthyrion_gatecases::build_world`
callee `<alloc::vec::IntoIter<SealedCap> as Iterator>::next` in `synthyrion_gatecases::build_world`
callee `<core::array::IntoIter<u8, 3> as Iterator>::next` in `synthyrion_gatecases::build_world`
callee `<core::iter::Map<Filter<Zip<core::slice::Iter<'_, SealedCap>, core::slice::Iter<'_, UntypedDesc>>, {closure@crates\synthyrion_gatecases\src\lib.rs:201:72: 201:80}>, {closure@crates\synthyrion_gatecases\src\lib.rs:201:124: 201:132}> as Iterator>::next` in `synthyrion_gatecases::build_world`
callee `<core::slice::Iter<'_, SealedCap> as Iterator>::zip` in `synthyrion_gatecases::build_world`
callee `BTreeSet::new` in `synthyrion_gatecases::build_world`
callee `Box::new` in `synthyrion_gatecases::build_world`
callee `Box::new_uninit` in `synthyrion_gatecases::build_world`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `synthyrion_gatecases::build_world`
callee `call (9 closure position(s) seen but 10 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::build_world`
callee `map_err (9 closure position(s) seen but 10 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::build_world`
callee `microkernel::Kernel::place_authorized` in `synthyrion_gatecases::build_world`
callee `microkernel::Kernel::sealed_sandbox_cap` in `synthyrion_gatecases::build_world`
callee `microkernel::Kernel::spawn` in `synthyrion_gatecases::build_world`
callee `microkernel::bindings::attach_vault_service` in `synthyrion_gatecases::build_world`
callee `microkernel::bindings::grant_vault` in `synthyrion_gatecases::build_world`
callee `microkernel::bootinfo::boot_root_task` in `synthyrion_gatecases::build_world`
callee `microkernel::cspace::cnode_install_initial` in `synthyrion_gatecases::build_world`
callee `microkernel::cspace::cs_lookup` in `synthyrion_gatecases::build_world`
callee `microkernel::cspace::set_task_cspace` in `synthyrion_gatecases::build_world`
callee `microkernel::gate::kernel_gate` in `synthyrion_gatecases::build_world`
callee `microkernel::iospace::cs_attach_iommu` in `synthyrion_gatecases::build_world`
callee `microkernel::iospace::record::iospace_install_table` in `synthyrion_gatecases::build_world`
callee `microkernel::iospace::retype_iospace` in `synthyrion_gatecases::build_world`
callee `microkernel::ipcx::create_notification` in `synthyrion_gatecases::build_world`
callee `microkernel::ipcx::sealed_schedule_cap` in `synthyrion_gatecases::build_world`
callee `microkernel::ipcx::set_partition_schedule_sealed` in `synthyrion_gatecases::build_world`
callee `microkernel::objects::map_anonymous_from` in `synthyrion_gatecases::build_world`
callee `microkernel::objects::map_frame` in `synthyrion_gatecases::build_world`
callee `microkernel::objects::retype` in `synthyrion_gatecases::build_world`
callee `microkernel::objects::seal_object_creation` in `synthyrion_gatecases::build_world`
callee `microkernel::objects::set_task_mcp_sealed` in `synthyrion_gatecases::build_world`
callee `microkernel::objects::vcpu_cap` in `synthyrion_gatecases::build_world`
callee `microkernel::vcpu::ept_create` in `synthyrion_gatecases::build_world`
callee `microkernel::vcpu::sealed_hv_cap` in `synthyrion_gatecases::build_world`
loop `synthyrion_gatecases::build_world` header bb376
callee `region_frames_for` in `synthyrion_gatecases::build_world::{closure#0}`
callee `<Option<usize> as Need<usize>>::need` in `synthyrion_gatecases::build_world::{closure#1}`
callee `<Result<SealedCap, WorldError> as FromResidual<Result<Infallible, WorldError>>>::from_residual` in `synthyrion_gatecases::build_world::{closure#1}`
callee `<Result<usize, WorldError> as Try>::branch` in `synthyrion_gatecases::build_world::{closure#1}`
callee `<Vec<SealedCap> as Deref>::deref` in `synthyrion_gatecases::build_world::{closure#1}`
callee `<Vec<SealedCap> as Index<usize>>::index` in `synthyrion_gatecases::build_world::{closure#1}`
callee `<Zip<core::slice::Iter<'_, SealedCap>, core::slice::Iter<'_, UntypedDesc>> as Iterator>::position` in `synthyrion_gatecases::build_world::{closure#1}`
callee `<core::slice::Iter<'_, SealedCap> as Iterator>::zip` in `synthyrion_gatecases::build_world::{closure#1}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::build_world::{closure#1}::{closure#0}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::build_world::{closure#2}`
callee `<Result<(), CsError> as Need<()>>::need` in `synthyrion_gatecases::build_world::{closure#4}`
callee `<Result<(), WorldError> as FromResidual<Result<Infallible, WorldError>>>::from_residual` in `synthyrion_gatecases::build_world::{closure#4}`
callee `<Result<(), WorldError> as Try>::branch` in `synthyrion_gatecases::build_world::{closure#4}`
callee `microkernel::cspace::cnode_install_initial` in `synthyrion_gatecases::build_world::{closure#4}`
callee `<core::str::Split<'_, {closure@crates\synthyrion_gatecases\src\lib.rs:906:23: 906:32}> as Iterator>::next` in `synthyrion_gatecases::cap_class`
callee `<str as PartialEq>::eq` in `synthyrion_gatecases::cap_class`
callee `core::str::split` in `synthyrion_gatecases::cap_class`
callee `<Vec<u64> as Deref>::deref` in `synthyrion_gatecases::extra_refusals`
callee `<Vec<u64> as IndexMut<usize>>::index_mut` in `synthyrion_gatecases::extra_refusals`
callee `Box::new_uninit` in `synthyrion_gatecases::extra_refusals`
callee `Vec::new` in `synthyrion_gatecases::extra_refusals`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `synthyrion_gatecases::extra_refusals`
callee `<After as PartialEq>::ne` in `synthyrion_gatecases::judge`
callee `<ChunksExact<'_, u8> as Iterator>::zip` in `synthyrion_gatecases::judge`
callee `<Enumerate<Zip<ChunksExact<'_, u8>, core::slice::Iter<'_, u64>>> as IntoIterator>::into_iter` in `synthyrion_gatecases::judge`
callee `<Enumerate<Zip<ChunksExact<'_, u8>, core::slice::Iter<'_, u64>>> as Iterator>::next` in `synthyrion_gatecases::judge`
callee `<Vec<u64> as Deref>::deref` in `synthyrion_gatecases::judge`
callee `<Vec<u8> as Deref>::deref` in `synthyrion_gatecases::judge`
callee `<Vec<u8> as DerefMut>::deref_mut` in `synthyrion_gatecases::judge`
callee `<Zip<ChunksExact<'_, u8>, core::slice::Iter<'_, u64>> as Iterator>::enumerate` in `synthyrion_gatecases::judge`
callee `<dyn UserMem as UserMem>::read` in `synthyrion_gatecases::judge`
callee `<u64 as Add<&u64>>::add` in `synthyrion_gatecases::judge`
callee `Arguments::new` in `synthyrion_gatecases::judge`
callee `Vec::len` in `synthyrion_gatecases::judge`
callee `alloc::vec::from_elem` in `synthyrion_gatecases::judge`
callee `core::fmt::rt::Argument::new_debug` in `synthyrion_gatecases::judge`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_gatecases::judge`
callee `core::fmt::rt::Argument::new_lower_hex` in `synthyrion_gatecases::judge`
callee `core::num::from_le_bytes` in `synthyrion_gatecases::judge`
callee `core::slice::chunks_exact` in `synthyrion_gatecases::judge`
callee `core::slice::copy_from_slice` in `synthyrion_gatecases::judge`
callee `format` in `synthyrion_gatecases::judge`
callee `must_use` in `synthyrion_gatecases::judge`
loop `synthyrion_gatecases::judge` header bb62
callee `<&Vec<(u64, Vec<u64>)> as IntoIterator>::into_iter` in `synthyrion_gatecases::prepare`
callee `<&Vec<u64> as IntoIterator>::into_iter` in `synthyrion_gatecases::prepare`
callee `<Vec<u8> as Deref>::deref` in `synthyrion_gatecases::prepare`
callee `<core::slice::Iter<'_, (u64, Vec<u64>)> as Iterator>::next` in `synthyrion_gatecases::prepare`
callee `<dyn UserMem as UserMem>::write` in `synthyrion_gatecases::prepare`
callee `<u64 as Add<&u64>>::add` in `synthyrion_gatecases::prepare`
callee `Vec::extend_from_slice` in `synthyrion_gatecases::prepare`
callee `Vec::len` in `synthyrion_gatecases::prepare`
callee `Vec::with_capacity` in `synthyrion_gatecases::prepare`
loop `synthyrion_gatecases::prepare` header bb2
callee `<ObjKind as PartialEq>::eq` in `synthyrion_gatecases::refusals`
callee `<Who as PartialEq>::ne` in `synthyrion_gatecases::refusals`
callee `<[Step] as Index<RangeTo<usize>>>::index` in `synthyrion_gatecases::refusals`
callee `<core::slice::Iter<'_, (u32, &str)> as Iterator>::any` in `synthyrion_gatecases::refusals`
callee `Arguments::new` in `synthyrion_gatecases::refusals`
callee `Box::new_uninit` in `synthyrion_gatecases::refusals`
callee `Vec::new` in `synthyrion_gatecases::refusals`
callee `Vec::push` in `synthyrion_gatecases::refusals`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `synthyrion_gatecases::refusals`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_gatecases::refusals`
callee `core::str::contains` in `synthyrion_gatecases::refusals`
callee `format` in `synthyrion_gatecases::refusals`
callee `must_use` in `synthyrion_gatecases::refusals`
callee `Arguments::new` in `synthyrion_gatecases::refusals::{closure#1}`
callee `Vec::push` in `synthyrion_gatecases::refusals::{closure#1}`
callee `alloc::slice::to_vec` in `synthyrion_gatecases::refusals::{closure#1}`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_gatecases::refusals::{closure#1}`
callee `format` in `synthyrion_gatecases::refusals::{closure#1}`
callee `must_use` in `synthyrion_gatecases::refusals::{closure#1}`
callee `<Vec<u64> as IndexMut<usize>>::index_mut` in `synthyrion_gatecases::regs_image`
callee `<core::iter::Map<core::ops::Range<u64>, {closure@crates\synthyrion_gatecases\src\lib.rs:898:42: 898:45}> as Iterator>::collect` in `synthyrion_gatecases::regs_image`
callee `<core::ops::Range<u64> as Iterator>::map` in `synthyrion_gatecases::regs_image`
callee `microkernel::cspace::cs_lookup` in `synthyrion_gatecases::resolve`
callee `<&Vec<Step> as IntoIterator>::into_iter` in `synthyrion_gatecases::run_row_host`
callee `<String as Deref>::deref` in `synthyrion_gatecases::run_row_host`
callee `<Vec<(String, Vec<Step>)> as IntoIterator>::into_iter` in `synthyrion_gatecases::run_row_host`
callee `<alloc::vec::IntoIter<(String, Vec<Step>)> as Iterator>::next` in `synthyrion_gatecases::run_row_host`
callee `<core::slice::Iter<'_, Step> as Iterator>::next` in `synthyrion_gatecases::run_row_host`
callee `Arguments::new` in `synthyrion_gatecases::run_row_host`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_gatecases::run_row_host`
callee `format` in `synthyrion_gatecases::run_row_host`
callee `microkernel::gate::kernel_gate` in `synthyrion_gatecases::run_row_host`
callee `must_use` in `synthyrion_gatecases::run_row_host`
loop `synthyrion_gatecases::run_row_host` header bb6
callee `<Vec<u64> as IndexMut<usize>>::index_mut` in `synthyrion_gatecases::rv_regs_image`
callee `<core::iter::Map<core::ops::Range<u64>, {closure@crates\synthyrion_gatecases\src\lib.rs:885:42: 885:45}> as Iterator>::collect` in `synthyrion_gatecases::rv_regs_image`
callee `<core::ops::Range<u64> as Iterator>::map` in `synthyrion_gatecases::rv_regs_image`
callee `<Enumerate<alloc::vec::IntoIter<Vec<Step>>> as IntoIterator>::into_iter` in `synthyrion_gatecases::scripts_of`
callee `<Enumerate<alloc::vec::IntoIter<Vec<Step>>> as Iterator>::next` in `synthyrion_gatecases::scripts_of`
callee `<Vec<Step> as Deref>::deref` in `synthyrion_gatecases::scripts_of`
callee `<Vec<Vec<Step>> as IntoIterator>::into_iter` in `synthyrion_gatecases::scripts_of`
callee `<alloc::vec::IntoIter<Vec<Step>> as Iterator>::enumerate` in `synthyrion_gatecases::scripts_of`
callee `Arguments::new` in `synthyrion_gatecases::scripts_of`
callee `Box::new_uninit` in `synthyrion_gatecases::scripts_of`
callee `Vec::push` in `synthyrion_gatecases::scripts_of`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `synthyrion_gatecases::scripts_of`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_gatecases::scripts_of`
callee `format` in `synthyrion_gatecases::scripts_of`
callee `must_use` in `synthyrion_gatecases::scripts_of`
loop `synthyrion_gatecases::scripts_of` header bb19
loop `synthyrion_gatecases::scripts_of` header bb41
callee `<String as Clone>::clone` in `synthyrion_gatecases::scripts_of::{closure#0}`
callee `<String as Clone>::clone` in `synthyrion_gatecases::scripts_of::{closure#1}`
callee `<Vec<u64> as Deref>::deref` in `synthyrion_gatecases::success_script`
callee `Box::new_uninit` in `synthyrion_gatecases::success_script`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `synthyrion_gatecases::success_script`
callee `alloc::slice::concat` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#10} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#11} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#12} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#13} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#14} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#15} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#16} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#2} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#3} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#4} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#5} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#6} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#7} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#8} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (closure success_script::{closure#9} is itself unbounded)` in `synthyrion_gatecases::success_script`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#10}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#11}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#12}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#13}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#14}`
callee `Box::new_uninit` in `synthyrion_gatecases::success_script::{closure#15}`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `synthyrion_gatecases::success_script::{closure#15}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#15}`
callee `Vec::push` in `synthyrion_gatecases::success_script::{closure#16}`
callee `call (2 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#16}`
loop `synthyrion_gatecases::success_script::{closure#16}` header bb9
callee `Box::new_uninit` in `synthyrion_gatecases::success_script::{closure#1}`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `synthyrion_gatecases::success_script::{closure#1}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#2}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#3}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#4}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#5}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#6}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#7}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#8}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_gatecases::success_script::{closure#9}`
callee `<Filter<core::slice::Iter<'_, CallDef>, {closure@crates\synthyrion_gatecases\src\lib.rs:998:35: 998:38}> as IntoIterator>::into_iter` in `synthyrion_gatecases::superseded_problems`
callee `<Filter<core::slice::Iter<'_, CallDef>, {closure@crates\synthyrion_gatecases\src\lib.rs:998:35: 998:38}> as Iterator>::next` in `synthyrion_gatecases::superseded_problems`
callee `<core::slice::Iter<'_, CallDef> as Iterator>::any` in `synthyrion_gatecases::superseded_problems`
callee `<core::slice::Iter<'_, CallDef> as Iterator>::filter` in `synthyrion_gatecases::superseded_problems`
callee `Arguments::new` in `synthyrion_gatecases::superseded_problems`
callee `Vec::new` in `synthyrion_gatecases::superseded_problems`
callee `Vec::push` in `synthyrion_gatecases::superseded_problems`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_gatecases::superseded_problems`
callee `core::str::contains` in `synthyrion_gatecases::superseded_problems`
callee `format` in `synthyrion_gatecases::superseded_problems`
callee `must_use` in `synthyrion_gatecases::superseded_problems`
loop `synthyrion_gatecases::superseded_problems` header bb5
callee `<Gate as PartialEq>::eq` in `synthyrion_gatecases::superseded_problems::{closure#0}`
callee `<Gate as PartialEq>::eq` in `synthyrion_gatecases::superseded_problems::{closure#1}`
callee `<Status as PartialEq>::eq` in `synthyrion_gatecases::superseded_problems::{closure#1}`
callee `<Vec<u64> as IndexMut<usize>>::index_mut` in `synthyrion_gatecases::vcpu_regs_image`
callee `<core::iter::Map<core::ops::Range<u64>, {closure@crates\synthyrion_gatecases\src\lib.rs:891:42: 891:45}> as Iterator>::collect` in `synthyrion_gatecases::vcpu_regs_image`
callee `<core::ops::Range<u64> as Iterator>::map` in `synthyrion_gatecases::vcpu_regs_image`
callee `<Filter<core::slice::Iter<'_, CallDef>, {closure@crates\synthyrion_gatecases\src\lib.rs:1012:25: 1012:28}> as Iterator>::collect` in `synthyrion_gatecases::wired_rows`
callee `<core::slice::Iter<'_, CallDef> as Iterator>::filter` in `synthyrion_gatecases::wired_rows`
callee `<Gate as PartialEq>::eq` in `synthyrion_gatecases::wired_rows::{closure#0}`
callee `<Status as PartialEq>::eq` in `synthyrion_gatecases::wired_rows::{closure#0}`
callee `<Result<(&mut microkernel::Kernel, Facts), WorldError> as FromResidual<Result<Infallible, WorldError>>>::from_residual` in `synthyrion_gatecases::world_for`
callee `<Result<Facts, WorldError> as Try>::branch` in `synthyrion_gatecases::world_for`
callee `microkernel::Kernel::init_in` in `synthyrion_gatecases::world_for`
loop `synthyrion_kcrypto::ct_eq` header bb4
callee `Formatter::debug_struct_field4_finish` in `synthyrion_kheal::<Event as Debug>::fmt`
callee `Formatter::debug_struct_field1_finish` in `synthyrion_kheal::<Fault as Debug>::fmt`
callee `Formatter::write_str` in `synthyrion_kheal::<Fault as Debug>::fmt`
callee `<&usize as PartialEq>::eq` in `synthyrion_kheal::<Fault as PartialEq>::eq`
callee `<[Option<Event>; L] as Clone>::clone` in `synthyrion_kheal::<FaultLog as Clone>::clone`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_kheal::<FaultLog as Debug>::fmt`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_kheal::<Guarded as Debug>::fmt`
callee `<Guarded<N, MASK> as Clone>::clone` in `synthyrion_kheal::<GuardedArr as Clone>::clone`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_kheal::<GuardedArr as Debug>::fmt`
callee `<Vec<u32> as Clone>::clone` in `synthyrion_kheal::<GuardedVec as Clone>::clone`
callee `<Vec<u64> as Clone>::clone` in `synthyrion_kheal::<GuardedVec as Clone>::clone`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_kheal::<GuardedVec as Debug>::fmt`
callee `Formatter::write_str` in `synthyrion_kheal::<Heal as Debug>::fmt`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_kheal::<KeyScrubber as Debug>::fmt`
callee `<u64 as Default>::default` in `synthyrion_kheal::<KeyScrubber as Default>::default`
callee `<usize as Default>::default` in `synthyrion_kheal::<KeyScrubber as Default>::default`
callee `Formatter::write_str` in `synthyrion_kheal::<Outcome as Debug>::fmt`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_kheal::<RegionStep as Debug>::fmt`
callee `<usize as Default>::default` in `synthyrion_kheal::<RegionStep as Default>::default`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_kheal::<ReloadRefused as Debug>::fmt`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_kheal::<ScrubStats as Debug>::fmt`
callee `<u64 as Default>::default` in `synthyrion_kheal::<ScrubStats as Default>::default`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_kheal::<Scrubber as Debug>::fmt`
callee `<u64 as Default>::default` in `synthyrion_kheal::<Scrubber as Default>::default`
callee `<usize as Default>::default` in `synthyrion_kheal::<Scrubber as Default>::default`
callee `Formatter::write_str` in `synthyrion_kheal::<ViewError as Debug>::fmt`
callee `core::slice::split_at` in `synthyrion_kheal::GuardedRef::open`
callee `Vec::capacity` in `synthyrion_kheal::GuardedVec::capacity`
callee `<&u64 as BitAnd<u64>>::bitand` in `synthyrion_kheal::GuardedVec::from_words`
callee `<Enumerate<Zip<core::slice::Iter<'_, u64>, core::slice::IterMut<'_, u32>>> as IntoIterator>::into_iter` in `synthyrion_kheal::GuardedVec::from_words`
callee `<Enumerate<Zip<core::slice::Iter<'_, u64>, core::slice::IterMut<'_, u32>>> as Iterator>::next` in `synthyrion_kheal::GuardedVec::from_words`
callee `<Zip<core::slice::Iter<'_, u64>, core::slice::IterMut<'_, u32>> as Iterator>::enumerate` in `synthyrion_kheal::GuardedVec::from_words`
callee `alloc::vec::from_elem` in `synthyrion_kheal::GuardedVec::from_words`
loop `synthyrion_kheal::GuardedVec::from_words` header bb10
callee `alloc::vec::from_elem` in `synthyrion_kheal::GuardedVec::new`
callee `<&u64 as BitAnd<u64>>::bitand` in `synthyrion_kheal::GuardedVec::reload_as`
callee `<Enumerate<core::slice::Iter<'_, u64>> as IntoIterator>::into_iter` in `synthyrion_kheal::GuardedVec::reload_as`
callee `<Enumerate<core::slice::Iter<'_, u64>> as Iterator>::next` in `synthyrion_kheal::GuardedVec::reload_as`
callee `<core::slice::Iter<'_, u64> as Iterator>::enumerate` in `synthyrion_kheal::GuardedVec::reload_as`
callee `Vec::clear` in `synthyrion_kheal::GuardedVec::reload_as`
callee `Vec::extend_from_slice` in `synthyrion_kheal::GuardedVec::reload_as`
callee `Vec::push` in `synthyrion_kheal::GuardedVec::reload_as`
loop `synthyrion_kheal::GuardedVec::reload_as` header bb8
callee `Vec::with_capacity` in `synthyrion_kheal::GuardedVec::with_capacity`
callee `<Result<GuardedView<'_, MASK>, ViewError> as FromResidual<Result<Infallible, ViewError>>>::from_residual` in `synthyrion_kheal::GuardedView::format`
callee `<Result<GuardedView<'_, MASK>, ViewError> as Try>::branch` in `synthyrion_kheal::GuardedView::format`
callee `<core::slice::IterMut<'_, u64> as IntoIterator>::into_iter` in `synthyrion_kheal::GuardedView::format`
callee `<core::slice::IterMut<'_, u64> as Iterator>::next` in `synthyrion_kheal::GuardedView::format`
loop `synthyrion_kheal::GuardedView::format` header bb8
loop `synthyrion_kheal::GuardedView::heal_parity` header bb4
callee `core::slice::split_at_mut` in `synthyrion_kheal::GuardedView::open`
loop `synthyrion_kheal::GuardedView::others_ok` header bb4
loop `synthyrion_kheal::GuardedView::scrub_range` header bb4
loop `synthyrion_kheal::GuardedView::xor_others` header bb1
callee `<F as FnMut<(usize, usize, usize)>>::call_mut` in `synthyrion_kheal::Scrubber::step`
loop `synthyrion_kheal::crc_tables` header bb1
loop `synthyrion_kheal::crc_tables` header bb16
callee `<Enumerate<core::slice::IterMut<'_, u32>> as IntoIterator>::into_iter` in `synthyrion_kheal::fill_zero_sums`
callee `<Enumerate<core::slice::IterMut<'_, u32>> as Iterator>::next` in `synthyrion_kheal::fill_zero_sums`
callee `<core::slice::IterMut<'_, u32> as Iterator>::enumerate` in `synthyrion_kheal::fill_zero_sums`
loop `synthyrion_kheal::fill_zero_sums` header bb5
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_pmm::<Frame as Debug>::fmt`
callee `<usize as Ord>::cmp` in `synthyrion_pmm::<Frame as Ord>::cmp`
callee `<usize as PartialOrd>::partial_cmp` in `synthyrion_pmm::<Frame as PartialOrd>::partial_cmp`
loop `synthyrion_pmm::FrameAllocator::alloc_contiguous` header bb5
loop `synthyrion_pmm::FrameAllocator::alloc_contiguous` header bb18
callee `alloc::vec::from_elem` in `synthyrion_pmm::FrameAllocator::new`
callee `core::num::div_ceil` in `synthyrion_pmm::FrameAllocator::new`
callee `core::num::is_multiple_of` in `synthyrion_pmm::FrameAllocator::new`
callee `core::slice::last_mut` in `synthyrion_pmm::FrameAllocator::new`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_ring::<Call as Debug>::fmt`
callee `<[u64; 4] as PartialEq>::eq` in `synthyrion_ring::<Call as PartialEq>::eq`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_ring::<Entered as Debug>::fmt`
callee `Formatter::write_str` in `synthyrion_ring::<GeometryError as Debug>::fmt`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_ring::<Ring as Debug>::fmt`
callee `<S as Server>::serve` in `synthyrion_ring::Ring::enter`
callee `<Vec<Option<usize>> as Clone>::clone` in `synthyrion_rv::<Automaton as Clone>::clone`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_rv::<Automaton as Debug>::fmt`
callee `<Vec<Option<usize>> as PartialEq>::eq` in `synthyrion_rv::<Automaton as PartialEq>::eq`
callee `<Vec<Option<usize>> as Clone>::clone` in `synthyrion_rv::<AutomatonBuilder as Clone>::clone`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_rv::<AutomatonBuilder as Debug>::fmt`
callee `<Option<Violation> as Clone>::clone` in `synthyrion_rv::<Monitor as Clone>::clone`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_rv::<Monitor as Debug>::fmt`
callee `Formatter::write_str` in `synthyrion_rv::<OnViolation as Debug>::fmt`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_rv::<RvError as Debug>::fmt`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_rv::<RvError as Debug>::fmt`
callee `<&Violation as PartialEq>::eq` in `synthyrion_rv::<RvError as PartialEq>::eq`
callee `<&usize as PartialEq>::eq` in `synthyrion_rv::<RvError as PartialEq>::eq`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_rv::<Violation as Debug>::fmt`
callee `alloc::vec::from_elem` in `synthyrion_rv::Automaton::builder`
callee `<Vec<Option<usize>> as Index<usize>>::index` in `synthyrion_rv::Automaton::next`
callee `<Filter<core::ops::Range<usize>, {closure@crates\synthyrion_rv\src\lib.rs:163:21: 163:25}> as Iterator>::collect` in `synthyrion_rv::Automaton::unreachable_events`
callee `<core::ops::Range<usize> as Iterator>::filter` in `synthyrion_rv::Automaton::unreachable_events`
callee `<core::ops::Range<usize> as Iterator>::all` in `synthyrion_rv::Automaton::unreachable_events::{closure#0}`
callee `<Filter<core::ops::Range<usize>, {closure@crates\synthyrion_rv\src\lib.rs:187:37: 187:41}> as Iterator>::collect` in `synthyrion_rv::Automaton::unreachable_states`
callee `<Vec<bool> as Index<usize>>::index` in `synthyrion_rv::Automaton::unreachable_states`
callee `<Vec<bool> as IndexMut<usize>>::index_mut` in `synthyrion_rv::Automaton::unreachable_states`
callee `<core::ops::Range<usize> as Iterator>::filter` in `synthyrion_rv::Automaton::unreachable_states`
callee `Box::new_uninit` in `synthyrion_rv::Automaton::unreachable_states`
callee `Vec::pop` in `synthyrion_rv::Automaton::unreachable_states`
callee `Vec::push` in `synthyrion_rv::Automaton::unreachable_states`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `synthyrion_rv::Automaton::unreachable_states`
callee `alloc::vec::from_elem` in `synthyrion_rv::Automaton::unreachable_states`
loop `synthyrion_rv::Automaton::unreachable_states` header bb6
callee `<Vec<bool> as Index<usize>>::index` in `synthyrion_rv::Automaton::unreachable_states::{closure#0}`
callee `Arguments::new` in `synthyrion_rv::AutomatonBuilder::build`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_rv::AutomatonBuilder::build`
callee `panic_fmt` in `synthyrion_rv::AutomatonBuilder::build`
callee `<Vec<Option<usize>> as Index<usize>>::index` in `synthyrion_rv::AutomatonBuilder::peek`
callee `<Vec<Option<usize>> as IndexMut<usize>>::index_mut` in `synthyrion_rv::AutomatonBuilder::transition`
callee `Arguments::new` in `synthyrion_rv::AutomatonBuilder::transition`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_rv::AutomatonBuilder::transition`
callee `panic_fmt` in `synthyrion_rv::AutomatonBuilder::transition`
callee `<&[usize] as IntoIterator>::into_iter` in `synthyrion_rv::Monitor::run`
callee `<Result<(), RvError> as FromResidual<Result<Infallible, RvError>>>::from_residual` in `synthyrion_rv::Monitor::run`
callee `<Result<usize, RvError> as Try>::branch` in `synthyrion_rv::Monitor::run`
callee `<core::slice::Iter<'_, usize> as Iterator>::next` in `synthyrion_rv::Monitor::run`
loop `synthyrion_rv::Monitor::run` header bb2
callee `Atomic::load` in `synthyrion_rv::embed::AtomicMonitor::events_accepted`
callee `Atomic::load` in `synthyrion_rv::embed::AtomicMonitor::first_violation`
callee `Atomic::new` in `synthyrion_rv::embed::AtomicMonitor::new`
callee `Atomic::fetch_add` in `synthyrion_rv::embed::AtomicMonitor::observe`
callee `Atomic::load` in `synthyrion_rv::embed::AtomicMonitor::observe`
callee `Atomic::store` in `synthyrion_rv::embed::AtomicMonitor::observe`
callee `Atomic::swap` in `synthyrion_rv::embed::AtomicMonitor::observe`
callee `Atomic::store` in `synthyrion_rv::embed::AtomicMonitor::reset`
callee `Atomic::load` in `synthyrion_rv::embed::AtomicMonitor::state`
callee `Atomic::load` in `synthyrion_rv::embed::AtomicMonitor::violations`
loop `synthyrion_rv::embed::EmbeddedSpec::is_well_formed` header bb11
callee `<&EmbeddedSpec as PartialEq>::eq` in `synthyrion_rv::embed::eq`
callee `<&[&str] as PartialEq>::eq` in `synthyrion_rv::embed::eq`
callee `<&[Option<u8>] as PartialEq>::eq` in `synthyrion_rv::embed::eq`
callee `<&str as PartialEq>::eq` in `synthyrion_rv::embed::eq`
callee `<Option<Violation> as PartialEq>::eq` in `synthyrion_rv::embed::eq`
callee `Formatter::debug_struct_field5_finish` in `synthyrion_rv::embed::fmt`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_rv::embed::fmt`
callee `<Enumerate<core::slice::Iter<'_, String>> as Iterator>::find_map` in `synthyrion_rv::first_duplicate`
callee `<core::slice::Iter<'_, String> as Iterator>::enumerate` in `synthyrion_rv::first_duplicate`
callee `<[String] as Index<RangeTo<usize>>>::index` in `synthyrion_rv::first_duplicate::{closure#0}`
callee `core::slice::contains` in `synthyrion_rv::first_duplicate::{closure#0}`
callee `<Enumerate<Lines<'_>> as IntoIterator>::into_iter` in `synthyrion_rv::parse`
callee `<Enumerate<Lines<'_>> as Iterator>::next` in `synthyrion_rv::parse`
callee `<Lines<'_> as Iterator>::enumerate` in `synthyrion_rv::parse`
callee `<Map<SplitWhitespace<'_>, {closure@crates\synthyrion_rv\src\spec.rs:128:50: 128:53}> as Iterator>::collect` in `synthyrion_rv::parse`
callee `<Map<SplitWhitespace<'_>, {closure@crates\synthyrion_rv\src\spec.rs:130:50: 130:53}> as Iterator>::collect` in `synthyrion_rv::parse`
callee `<Result<Spec, SpecError> as FromResidual<Result<Infallible, SpecError>>>::from_residual` in `synthyrion_rv::parse`
callee `<Result<usize, SpecError> as Try>::branch` in `synthyrion_rv::parse`
callee `<SplitWhitespace<'_> as Iterator>::map` in `synthyrion_rv::parse`
callee `<String as Deref>::deref` in `synthyrion_rv::parse`
callee `<String as From<&str>>::from` in `synthyrion_rv::parse`
callee `<Vec<(usize, String, String, String)> as IntoIterator>::into_iter` in `synthyrion_rv::parse`
callee `<Vec<String> as Deref>::deref` in `synthyrion_rv::parse`
callee `<alloc::vec::IntoIter<(usize, String, String, String)> as Iterator>::next` in `synthyrion_rv::parse`
callee `<core::str::Split<'_, char> as Iterator>::next` in `synthyrion_rv::parse`
callee `<str as ToString>::to_string` in `synthyrion_rv::parse`
callee `Arguments::new` in `synthyrion_rv::parse`
callee `Vec::is_empty` in `synthyrion_rv::parse`
callee `Vec::len` in `synthyrion_rv::parse`
callee `Vec::new` in `synthyrion_rv::parse`
callee `Vec::push` in `synthyrion_rv::parse`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_rv::parse`
callee `core::str::is_empty` in `synthyrion_rv::parse`
callee `core::str::lines` in `synthyrion_rv::parse`
callee `core::str::split` in `synthyrion_rv::parse`
callee `core::str::split_once` in `synthyrion_rv::parse`
callee `core::str::split_whitespace` in `synthyrion_rv::parse`
callee `core::str::strip_prefix` in `synthyrion_rv::parse`
callee `core::str::trim` in `synthyrion_rv::parse`
callee `format` in `synthyrion_rv::parse`
callee `must_use` in `synthyrion_rv::parse`
loop `synthyrion_rv::parse` header bb8
loop `synthyrion_rv::parse` header bb101
callee `<str as ToString>::to_string` in `synthyrion_rv::parse::{closure#0}`
callee `<str as ToString>::to_string` in `synthyrion_rv::parse::{closure#1}`
callee `<Vec<String> as Deref>::deref` in `synthyrion_rv::parse::{closure#2}`
callee `<core::slice::Iter<'_, String> as Iterator>::position` in `synthyrion_rv::parse::{closure#2}`
callee `<&String as PartialEq<&str>>::eq` in `synthyrion_rv::parse::{closure#2}::{closure#0}`
callee `<String as Clone>::clone` in `synthyrion_rv::parse::{closure#3}`
callee `<Vec<String> as Index<usize>>::index` in `synthyrion_rv::parse::{closure#3}`
callee `Arguments::new` in `synthyrion_rv::parse::{closure#4}`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_rv::parse::{closure#4}`
callee `format` in `synthyrion_rv::parse::{closure#4}`
callee `must_use` in `synthyrion_rv::parse::{closure#4}`
callee `Arguments::new` in `synthyrion_rv::parse::{closure#5}`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_rv::parse::{closure#5}`
callee `format` in `synthyrion_rv::parse::{closure#5}`
callee `must_use` in `synthyrion_rv::parse::{closure#5}`
callee `Arguments::new` in `synthyrion_rv::parse::{closure#6}`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_rv::parse::{closure#6}`
callee `format` in `synthyrion_rv::parse::{closure#6}`
callee `must_use` in `synthyrion_rv::parse::{closure#6}`
callee `Arguments::new` in `synthyrion_rv::parse::{closure#7}`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_rv::parse::{closure#7}`
callee `format` in `synthyrion_rv::parse::{closure#7}`
callee `must_use` in `synthyrion_rv::parse::{closure#7}`
callee `<core::slice::Iter<'_, &EmbeddedSpec> as Iterator>::all` in `synthyrion_rv::runtime_verification_is_compiled_in`
callee `Arguments::new` in `synthyrion_rv::spec::Spec::describe`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_rv::spec::Spec::describe`
callee `format` in `synthyrion_rv::spec::Spec::describe`
callee `must_use` in `synthyrion_rv::spec::Spec::describe`
callee `<Map<core::slice::Iter<'_, &str>, {closure@crates\synthyrion_rv\src\spec.rs:89:18: 89:21}> as Iterator>::collect` in `synthyrion_rv::spec::Spec::encode_trace`
callee `<core::slice::Iter<'_, &str> as Iterator>::map` in `synthyrion_rv::spec::Spec::encode_trace`
callee `ok_or_else (closure spec::Spec::encode_trace::{closure#0}::{closure#0} is itself unbounded)` in `synthyrion_rv::spec::Spec::encode_trace::{closure#0}`
callee `<Vec<String> as Deref>::deref` in `synthyrion_rv::spec::Spec::encode_trace::{closure#0}::{closure#0}`
callee `Arguments::new` in `synthyrion_rv::spec::Spec::encode_trace::{closure#0}::{closure#0}`
callee `alloc::slice::join` in `synthyrion_rv::spec::Spec::encode_trace::{closure#0}::{closure#0}`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_rv::spec::Spec::encode_trace::{closure#0}::{closure#0}`
callee `format` in `synthyrion_rv::spec::Spec::encode_trace::{closure#0}::{closure#0}`
callee `must_use` in `synthyrion_rv::spec::Spec::encode_trace::{closure#0}::{closure#0}`
callee `<Vec<String> as Deref>::deref` in `synthyrion_rv::spec::Spec::event_id`
callee `<core::slice::Iter<'_, String> as Iterator>::position` in `synthyrion_rv::spec::Spec::event_id`
callee `<&String as PartialEq<&str>>::eq` in `synthyrion_rv::spec::Spec::event_id::{closure#0}`
callee `<Vec<String> as Deref>::deref` in `synthyrion_rv::spec::Spec::event_name`
callee `core::slice::get` in `synthyrion_rv::spec::Spec::event_name`
callee `String::as_str` in `synthyrion_rv::spec::Spec::event_name::{closure#0}`
callee `<Vec<String> as Deref>::deref` in `synthyrion_rv::spec::Spec::state_id`
callee `<core::slice::Iter<'_, String> as Iterator>::position` in `synthyrion_rv::spec::Spec::state_id`
callee `<&String as PartialEq<&str>>::eq` in `synthyrion_rv::spec::Spec::state_id::{closure#0}`
callee `<Vec<String> as Deref>::deref` in `synthyrion_rv::spec::Spec::state_name`
callee `core::slice::get` in `synthyrion_rv::spec::Spec::state_name`
callee `String::as_str` in `synthyrion_rv::spec::Spec::state_name::{closure#0}`
callee `<impl Into<String> as Into<String>>::into` in `synthyrion_rv::spec::SpecError::at`
callee `<String as Clone>::clone` in `synthyrion_rv::spec::clone`
callee `<Vec<String> as Clone>::clone` in `synthyrion_rv::spec::clone`
callee `<String as PartialEq>::eq` in `synthyrion_rv::spec::eq`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_rv::spec::fmt`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_rv::spec::fmt`
callee `Formatter::write_str` in `synthyrion_sched_rt::<Policy as Debug>::fmt`
callee `<Vec<(u32, u32)> as Clone>::clone` in `synthyrion_sched_rt::<PriorityInheritance as Clone>::clone`
callee `<Vec<(u32, u32, u64)> as Clone>::clone` in `synthyrion_sched_rt::<PriorityInheritance as Clone>::clone`
callee `<Vec<(u32, u64)> as Clone>::clone` in `synthyrion_sched_rt::<PriorityInheritance as Clone>::clone`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_sched_rt::<PriorityInheritance as Debug>::fmt`
callee `<Vec<(u32, u32)> as Default>::default` in `synthyrion_sched_rt::<PriorityInheritance as Default>::default`
callee `<Vec<(u32, u32, u64)> as Default>::default` in `synthyrion_sched_rt::<PriorityInheritance as Default>::default`
callee `<Vec<(u32, u64)> as Default>::default` in `synthyrion_sched_rt::<PriorityInheritance as Default>::default`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_sched_rt::<Ready as Debug>::fmt`
callee `<Option<u32> as PartialEq>::eq` in `synthyrion_sched_rt::<Ready as PartialEq>::eq`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_sched_rt::<RtTask as Debug>::fmt`
callee `Formatter::debug_struct_field1_finish` in `synthyrion_sched_rt::<Unschedulable as Debug>::fmt`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_sched_rt::<Unschedulable as Debug>::fmt`
callee `<&u32 as PartialEq>::eq` in `synthyrion_sched_rt::<Unschedulable as PartialEq>::eq`
callee `<&u64 as PartialEq>::eq` in `synthyrion_sched_rt::<Unschedulable as PartialEq>::eq`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_sched_rt::<Violation as Debug>::fmt`
callee `<&u32 as PartialEq>::eq` in `synthyrion_sched_rt::<Violation as PartialEq>::eq`
callee `<&u64 as PartialEq>::eq` in `synthyrion_sched_rt::<Violation as PartialEq>::eq`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_sched_rt::<WcetBasis as Debug>::fmt`
callee `Formatter::write_str` in `synthyrion_sched_rt::<WcetBasis as Debug>::fmt`
callee `<&&str as PartialEq>::eq` in `synthyrion_sched_rt::<WcetBasis as PartialEq>::eq`
callee `Vec::push` in `synthyrion_sched_rt::PriorityInheritance::acquire`
callee `Vec::retain` in `synthyrion_sched_rt::PriorityInheritance::acquire`
callee `<Vec<(u32, u32)> as Deref>::deref` in `synthyrion_sched_rt::PriorityInheritance::block_on`
callee `<Vec<(u32, u32, u64)> as Deref>::deref` in `synthyrion_sched_rt::PriorityInheritance::block_on`
callee `<Vec<(u32, u64)> as DerefMut>::deref_mut` in `synthyrion_sched_rt::PriorityInheritance::block_on`
callee `<core::slice::Iter<'_, (u32, u32)> as Iterator>::find` in `synthyrion_sched_rt::PriorityInheritance::block_on`
callee `<core::slice::Iter<'_, (u32, u32, u64)> as Iterator>::find` in `synthyrion_sched_rt::PriorityInheritance::block_on`
callee `<core::slice::IterMut<'_, (u32, u64)> as Iterator>::find` in `synthyrion_sched_rt::PriorityInheritance::block_on`
callee `Vec::push` in `synthyrion_sched_rt::PriorityInheritance::block_on`
callee `Vec::retain` in `synthyrion_sched_rt::PriorityInheritance::block_on`
loop `synthyrion_sched_rt::PriorityInheritance::block_on` header bb12
callee `<Vec<(u32, u64)> as Deref>::deref` in `synthyrion_sched_rt::PriorityInheritance::effective_priority`
callee `<core::slice::Iter<'_, (u32, u64)> as Iterator>::find` in `synthyrion_sched_rt::PriorityInheritance::effective_priority`
callee `core::cmp::min` in `synthyrion_sched_rt::PriorityInheritance::effective_priority::{closure#1}`
callee `<Vec<(u32, u32, u64)> as Deref>::deref` in `synthyrion_sched_rt::PriorityInheritance::release`
callee `<core::slice::Iter<'_, (u32, u32, u64)> as Iterator>::any` in `synthyrion_sched_rt::PriorityInheritance::release`
callee `Vec::retain` in `synthyrion_sched_rt::PriorityInheritance::release`
callee `<&[RtTask] as IntoIterator>::into_iter` in `synthyrion_sched_rt::admit_for_execution`
callee `<core::slice::Iter<'_, RtTask> as Iterator>::next` in `synthyrion_sched_rt::admit_for_execution`
loop `synthyrion_sched_rt::admit_for_execution` header bb2
callee `<Result<(), Unschedulable> as FromResidual<Result<Infallible, Unschedulable>>>::from_residual` in `synthyrion_sched_rt::edf_schedulable`
callee `<Result<(), Unschedulable> as Try>::branch` in `synthyrion_sched_rt::edf_schedulable`
callee `<core::slice::Iter<'_, Ready> as Iterator>::min_by_key` in `synthyrion_sched_rt::pick`
callee `<Result<(), Unschedulable> as FromResidual<Result<Infallible, Unschedulable>>>::from_residual` in `synthyrion_sched_rt::rms_schedulable`
callee `<Result<(), Unschedulable> as Try>::branch` in `synthyrion_sched_rt::rms_schedulable`
callee `<Filter<core::slice::Iter<'_, RtTask>, {closure@crates\synthyrion_sched_rt\src\lib.rs:456:21: 456:24}> as Iterator>::max_by_key` in `synthyrion_sched_rt::shed_to_fit`
callee `<Vec<RtTask> as Deref>::deref` in `synthyrion_sched_rt::shed_to_fit`
callee `<core::slice::Iter<'_, RtTask> as Iterator>::filter` in `synthyrion_sched_rt::shed_to_fit`
callee `Vec::retain` in `synthyrion_sched_rt::shed_to_fit`
callee `alloc::slice::to_vec` in `synthyrion_sched_rt::shed_to_fit`
callee `call (4 closure position(s) seen but 5 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_sched_rt::shed_to_fit`
callee `map (4 closure position(s) seen but 5 closure body(ies) owned: the ordinal is not resolvable)` in `synthyrion_sched_rt::shed_to_fit`
loop `synthyrion_sched_rt::shed_to_fit` header bb2
callee `Result::is_ok` in `synthyrion_sched_rt::shed_to_fit::{closure#0}`
callee `<Map<core::slice::Iter<'_, (u64, u64)>, {closure@crates\synthyrion_sched_rt\src\sporadic.rs:155:41: 155:55}> as Iterator>::min` in `synthyrion_sched_rt::sporadic::Refills::next_release`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::map` in `synthyrion_sched_rt::sporadic::Refills::next_release`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::fold` in `synthyrion_sched_rt::sporadic::Refills::total`
callee `<[(u64, u64); 8] as PartialEq>::eq` in `synthyrion_sched_rt::sporadic::eq`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_sched_rt::sporadic::fmt`
callee `<Map<core::slice::Iter<'_, RtTask>, {closure@crates\synthyrion_sched_rt\src\lib.rs:188:22: 188:25}> as Iterator>::sum` in `synthyrion_sched_rt::total_utilisation_permille`
callee `<core::slice::Iter<'_, RtTask> as Iterator>::map` in `synthyrion_sched_rt::total_utilisation_permille`
callee `<&[RtTask] as IntoIterator>::into_iter` in `synthyrion_sched_rt::validate`
callee `<core::slice::Iter<'_, RtTask> as Iterator>::next` in `synthyrion_sched_rt::validate`
loop `synthyrion_sched_rt::validate` header bb2
loop `synthyrion_sched_rt::validate` header bb11
callee `<Self as Clock>::now` in `synthyrion_smp::Clock::spin_until`
loop `synthyrion_smp::Clock::spin_until` header bb1
callee `<&Vec<Cpu> as IntoIterator>::into_iter` in `synthyrion_smp::ap_targets`
callee `<Vec<u32> as Deref>::deref` in `synthyrion_smp::ap_targets`
callee `<core::slice::Iter<'_, Cpu> as Iterator>::next` in `synthyrion_smp::ap_targets`
callee `Vec::new` in `synthyrion_smp::ap_targets`
callee `Vec::push` in `synthyrion_smp::ap_targets`
callee `core::slice::contains` in `synthyrion_smp::ap_targets`
loop `synthyrion_smp::ap_targets` header bb3
callee `<&Vec<Cpu> as IntoIterator>::into_iter` in `synthyrion_smp::ap_targets_x2apic`
callee `<Vec<u32> as Deref>::deref` in `synthyrion_smp::ap_targets_x2apic`
callee `<core::slice::Iter<'_, Cpu> as Iterator>::next` in `synthyrion_smp::ap_targets_x2apic`
callee `Vec::new` in `synthyrion_smp::ap_targets_x2apic`
callee `Vec::push` in `synthyrion_smp::ap_targets_x2apic`
callee `core::slice::contains` in `synthyrion_smp::ap_targets_x2apic`
loop `synthyrion_smp::ap_targets_x2apic` header bb3
callee `<PrefetchPolicy as PartialEq>::eq` in `synthyrion_smp::apply_prefetch_policy`
callee `<impl MsrAccess as MsrAccess>::rdmsr` in `synthyrion_smp::apply_prefetch_policy`
callee `<impl MsrAccess as MsrAccess>::wrmsr` in `synthyrion_smp::apply_prefetch_policy`
callee `<Vec<u64> as DerefMut>::deref_mut` in `synthyrion_smp::budget_from`
callee `<Vec<u64> as Index<usize>>::index` in `synthyrion_smp::budget_from`
callee `<usize as Ord>::min` in `synthyrion_smp::budget_from`
callee `Vec::len` in `synthyrion_smp::budget_from`
callee `alloc::slice::to_vec` in `synthyrion_smp::budget_from`
callee `core::slice::sort_unstable` in `synthyrion_smp::budget_from`
callee `core::num::next_power_of_two` in `synthyrion_smp::cache_is_core_private`
callee `Arguments::new` in `synthyrion_smp::calib_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::calib_line`
callee `format` in `synthyrion_smp::calib_line`
callee `must_use` in `synthyrion_smp::calib_line`
callee `<core::ops::Range<u32> as IntoIterator>::into_iter` in `synthyrion_smp::check_colour_bits`
callee `<core::ops::Range<u32> as Iterator>::next` in `synthyrion_smp::check_colour_bits`
callee `<core::slice::Iter<'_, BitProbe> as Iterator>::find` in `synthyrion_smp::check_colour_bits`
loop `synthyrion_smp::check_colour_bits` header bb9
callee `<Enumerate<core::slice::Iter<'_, (u32, u64)>> as IntoIterator>::into_iter` in `synthyrion_smp::check_smt_confinement`
callee `<Enumerate<core::slice::Iter<'_, (u32, u64)>> as Iterator>::next` in `synthyrion_smp::check_smt_confinement`
callee `<Enumerate<core::slice::Iter<'_, (u32, u64)>> as Iterator>::skip` in `synthyrion_smp::check_smt_confinement`
callee `<Skip<Enumerate<core::slice::Iter<'_, (u32, u64)>>> as IntoIterator>::into_iter` in `synthyrion_smp::check_smt_confinement`
callee `<Skip<Enumerate<core::slice::Iter<'_, (u32, u64)>>> as Iterator>::next` in `synthyrion_smp::check_smt_confinement`
callee `<core::slice::Iter<'_, (u32, u64)> as Iterator>::enumerate` in `synthyrion_smp::check_smt_confinement`
loop `synthyrion_smp::check_smt_confinement` header bb4
callee `<alloc::collections::btree_map::Iter<'_, u64, u64> as Iterator>::find` in `synthyrion_smp::colour::ColourPolicy::assign`
callee `BTreeMap::insert` in `synthyrion_smp::colour::ColourPolicy::assign`
callee `BTreeMap::iter` in `synthyrion_smp::colour::ColourPolicy::assign`
callee `BTreeMap::new` in `synthyrion_smp::colour::ColourPolicy::with_shift`
callee `<BTreeMap<u64, u64> as Clone>::clone` in `synthyrion_smp::colour::clone`
callee `<&u32 as PartialEq>::eq` in `synthyrion_smp::colour::eq`
callee `<&u64 as PartialEq>::eq` in `synthyrion_smp::colour::eq`
callee `Formatter::debug_struct_field1_finish` in `synthyrion_smp::colour::fmt`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_smp::colour::fmt`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_smp::colour::fmt`
callee `Formatter::write_str` in `synthyrion_smp::colour::fmt`
callee `core::num::leading_zeros` in `synthyrion_smp::colour_bits_from_leaf4`
callee `<core::slice::Iter<'_, u32> as Iterator>::position` in `synthyrion_smp::core_index`
loop `synthyrion_smp::detect` header bb3
loop `synthyrion_smp::detect` header bb8
callee `Vec::len` in `synthyrion_smp::domain_pages`
callee `Vec::push` in `synthyrion_smp::domain_pages`
callee `Vec::with_capacity` in `synthyrion_smp::domain_pages`
loop `synthyrion_smp::domain_pages` header bb2
callee `<FlushDone as Default>::default` in `synthyrion_smp::domain_switch_flush`
callee `<impl FlushMachine as FlushMachine>::l1d_sweep` in `synthyrion_smp::domain_switch_flush`
callee `<impl FlushMachine as FlushMachine>::l2_sweep` in `synthyrion_smp::domain_switch_flush`
callee `<impl FlushMachine as FlushMachine>::llc_wbinvd` in `synthyrion_smp::domain_switch_flush`
callee `<impl FlushMachine as FlushMachine>::tlb_flush` in `synthyrion_smp::domain_switch_flush`
callee `<impl FlushMachine as FlushMachine>::verw_md_clear` in `synthyrion_smp::domain_switch_flush`
callee `<impl FlushMachine as FlushMachine>::wrmsr` in `synthyrion_smp::domain_switch_flush`
callee `<Enumerate<core::slice::Iter<'_, &str>> as IntoIterator>::into_iter` in `synthyrion_smp::final_line`
callee `<Enumerate<core::slice::Iter<'_, &str>> as Iterator>::next` in `synthyrion_smp::final_line`
callee `<String as Deref>::deref` in `synthyrion_smp::final_line`
callee `<Vec<&str> as Deref>::deref` in `synthyrion_smp::final_line`
callee `<core::slice::Iter<'_, &str> as Iterator>::enumerate` in `synthyrion_smp::final_line`
callee `<core::str::Split<'_, char> as Iterator>::collect` in `synthyrion_smp::final_line`
callee `Arguments::new` in `synthyrion_smp::final_line`
callee `String::is_empty` in `synthyrion_smp::final_line`
callee `String::len` in `synthyrion_smp::final_line`
callee `String::new` in `synthyrion_smp::final_line`
callee `String::push` in `synthyrion_smp::final_line`
callee `String::push_str` in `synthyrion_smp::final_line`
callee `Vec::len` in `synthyrion_smp::final_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::final_line`
callee `core::str::is_empty` in `synthyrion_smp::final_line`
callee `core::str::len` in `synthyrion_smp::final_line`
callee `core::str::split` in `synthyrion_smp::final_line`
callee `format` in `synthyrion_smp::final_line`
callee `must_use` in `synthyrion_smp::final_line`
loop `synthyrion_smp::final_line` header bb30
callee `<Enumerate<core::slice::Iter<'_, u32>> as Iterator>::all` in `synthyrion_smp::ids_are_distinct`
callee `<core::slice::Iter<'_, u32> as Iterator>::enumerate` in `synthyrion_smp::ids_are_distinct`
callee `<[u32] as Index<core::ops::RangeFrom<usize>>>::index` in `synthyrion_smp::ids_are_distinct::{closure#0}`
callee `<core::slice::Iter<'_, u32> as Iterator>::all` in `synthyrion_smp::ids_are_distinct::{closure#0}`
callee `<&u32 as PartialEq>::ne` in `synthyrion_smp::ids_are_distinct::{closure#0}::{closure#0}`
callee `core::num::leading_zeros` in `synthyrion_smp::isqrt`
loop `synthyrion_smp::isqrt` header bb7
callee `<Option<&[u8]> as Try>::branch` in `synthyrion_smp::le32`
callee `core::num::from_le_bytes` in `synthyrion_smp::le32`
callee `core::slice::get` in `synthyrion_smp::le32`
callee `<Option<&[u8]> as Try>::branch` in `synthyrion_smp::le64`
callee `core::num::from_le_bytes` in `synthyrion_smp::le64`
callee `core::slice::get` in `synthyrion_smp::le64`
callee `<Q16 as PartialOrd>::gt` in `synthyrion_smp::leakage::ScenarioVerdict::bits_per_trial`
callee `<Vec<(u64, bool)> as Deref>::deref` in `synthyrion_smp::leakage::SimCache::access`
callee `<Vec<Vec<(u64, bool)>> as IndexMut<usize>>::index_mut` in `synthyrion_smp::leakage::SimCache::access`
callee `<core::slice::Iter<'_, (u64, bool)> as Iterator>::position` in `synthyrion_smp::leakage::SimCache::access`
callee `Vec::len` in `synthyrion_smp::leakage::SimCache::access`
callee `Vec::push` in `synthyrion_smp::leakage::SimCache::access`
callee `Vec::remove` in `synthyrion_smp::leakage::SimCache::access`
callee `<Vec<(u64, bool)> as Deref>::deref` in `synthyrion_smp::leakage::SimCache::contains`
callee `<Vec<Vec<(u64, bool)>> as Index<usize>>::index` in `synthyrion_smp::leakage::SimCache::contains`
callee `<core::slice::Iter<'_, (u64, bool)> as Iterator>::any` in `synthyrion_smp::leakage::SimCache::contains`
callee `<Filter<Flatten<core::slice::Iter<'_, Vec<(u64, bool)>>>, {closure@crates\synthyrion_smp\src\leakage.rs:265:44: 265:54}> as Iterator>::count` in `synthyrion_smp::leakage::SimCache::dirty`
callee `<Flatten<core::slice::Iter<'_, Vec<(u64, bool)>>> as Iterator>::filter` in `synthyrion_smp::leakage::SimCache::dirty`
callee `<Vec<Vec<(u64, bool)>> as Deref>::deref` in `synthyrion_smp::leakage::SimCache::dirty`
callee `<core::slice::Iter<'_, Vec<(u64, bool)>> as Iterator>::flatten` in `synthyrion_smp::leakage::SimCache::dirty`
callee `<Vec<Vec<(u64, bool)>> as DerefMut>::deref_mut` in `synthyrion_smp::leakage::SimCache::flush`
callee `<core::slice::IterMut<'_, Vec<(u64, bool)>> as IntoIterator>::into_iter` in `synthyrion_smp::leakage::SimCache::flush`
callee `<core::slice::IterMut<'_, Vec<(u64, bool)>> as Iterator>::next` in `synthyrion_smp::leakage::SimCache::flush`
callee `Vec::clear` in `synthyrion_smp::leakage::SimCache::flush`
loop `synthyrion_smp::leakage::SimCache::flush` header bb5
callee `Vec::new` in `synthyrion_smp::leakage::SimCache::new`
callee `alloc::vec::from_elem` in `synthyrion_smp::leakage::SimCache::new`
callee `Vec::new` in `synthyrion_smp::leakage::SimCache::new_sliced`
callee `alloc::vec::from_elem` in `synthyrion_smp::leakage::SimCache::new_sliced`
callee `<Option<(u32, u64, u64)> as Clone>::clone` in `synthyrion_smp::leakage::clone`
callee `<Option<FlushPlan> as Clone>::clone` in `synthyrion_smp::leakage::clone`
callee `<Option<SwitchPad> as Clone>::clone` in `synthyrion_smp::leakage::clone`
callee `<PtPlacement as Clone>::clone` in `synthyrion_smp::leakage::clone`
callee `<SenderList as Clone>::clone` in `synthyrion_smp::leakage::clone`
callee `<Vec<Vec<(u64, bool)>> as Clone>::clone` in `synthyrion_smp::leakage::clone`
callee `<u64 as Ord>::cmp` in `synthyrion_smp::leakage::cmp`
callee `<u64 as Default>::default` in `synthyrion_smp::leakage::default`
callee `<&usize as PartialEq>::eq` in `synthyrion_smp::leakage::eq`
callee `<Q16 as PartialEq>::eq` in `synthyrion_smp::leakage::eq`
callee `Arguments::new` in `synthyrion_smp::leakage::fmt`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_smp::leakage::fmt`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_smp::leakage::fmt`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_smp::leakage::fmt`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_smp::leakage::fmt`
callee `Formatter::write_fmt` in `synthyrion_smp::leakage::fmt`
callee `Formatter::write_str` in `synthyrion_smp::leakage::fmt`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::leakage::fmt`
callee `<u64 as PartialOrd>::partial_cmp` in `synthyrion_smp::leakage::partial_cmp`
callee `<&[(u32, u32)] as IntoIterator>::into_iter` in `synthyrion_smp::llc_bits_line`
callee `<String as Deref>::deref` in `synthyrion_smp::llc_bits_line`
callee `<core::slice::Iter<'_, (u32, u32)> as Iterator>::next` in `synthyrion_smp::llc_bits_line`
callee `Arguments::new` in `synthyrion_smp::llc_bits_line`
callee `String::push_str` in `synthyrion_smp::llc_bits_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::llc_bits_line`
callee `format` in `synthyrion_smp::llc_bits_line`
callee `must_use` in `synthyrion_smp::llc_bits_line`
loop `synthyrion_smp::llc_bits_line` header bb6
callee `Arguments::new` in `synthyrion_smp::llc_cal_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::llc_cal_line`
callee `format` in `synthyrion_smp::llc_cal_line`
callee `must_use` in `synthyrion_smp::llc_cal_line`
callee `Arguments::new` in `synthyrion_smp::llc_cat_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::llc_cat_line`
callee `core::fmt::rt::Argument::new_lower_hex` in `synthyrion_smp::llc_cat_line`
callee `format` in `synthyrion_smp::llc_cat_line`
callee `must_use` in `synthyrion_smp::llc_cat_line`
callee `<u8 as From<bool>>::from` in `synthyrion_smp::llc_dx_line`
callee `Arguments::new` in `synthyrion_smp::llc_dx_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::llc_dx_line`
callee `format` in `synthyrion_smp::llc_dx_line`
callee `must_use` in `synthyrion_smp::llc_dx_line`
callee `Arguments::new` in `synthyrion_smp::llc_verdict_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::llc_verdict_line`
callee `format` in `synthyrion_smp::llc_verdict_line`
callee `must_use` in `synthyrion_smp::llc_verdict_line`
callee `Arguments::new` in `synthyrion_smp::llc_walk_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::llc_walk_line`
callee `core::fmt::rt::Argument::new_lower_hex` in `synthyrion_smp::llc_walk_line`
callee `format` in `synthyrion_smp::llc_walk_line`
callee `must_use` in `synthyrion_smp::llc_walk_line`
callee `Arguments::new` in `synthyrion_smp::llc_wbcal_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::llc_wbcal_line`
callee `format` in `synthyrion_smp::llc_wbcal_line`
callee `must_use` in `synthyrion_smp::llc_wbcal_line`
callee `Atomic::new` in `synthyrion_smp::lock::TicketLock::from_parts`
callee `Atomic::load` in `synthyrion_smp::lock::TicketLock::is_held`
loop `synthyrion_smp::lock::TicketLock::lock_offering` header bb2
callee `Atomic::new` in `synthyrion_smp::lock::TicketLock::new`
callee `Atomic::load` in `synthyrion_smp::lock::TicketLock::ready`
callee `Atomic::load` in `synthyrion_smp::lock::TicketLock::snapshot`
callee `Atomic::fetch_add` in `synthyrion_smp::lock::TicketLock::take`
callee `Atomic::load` in `synthyrion_smp::lock::TicketLock::tickets_issued`
callee `Atomic::compare_exchange` in `synthyrion_smp::lock::TicketLock::try_cas`
callee `Atomic::load` in `synthyrion_smp::lock::TicketLock::try_read`
callee `Atomic::load` in `synthyrion_smp::lock::TicketLock::unlock`
callee `Atomic::store` in `synthyrion_smp::lock::TicketLock::unlock`
callee `<dyn FnMut() as FnMut<()>>::call_mut` in `synthyrion_smp::lock::TicketLock::wait_poll`
callee `spin_loop` in `synthyrion_smp::lock::TicketLock::wait_poll`
callee `Formatter::write_str` in `synthyrion_smp::lock::fmt`
callee `<core::ops::RangeInclusive<i32> as IntoIterator>::into_iter` in `synthyrion_smp::log2_q16`
callee `<core::ops::RangeInclusive<i32> as Iterator>::next` in `synthyrion_smp::log2_q16`
callee `core::num::leading_zeros` in `synthyrion_smp::log2_q16`
callee `core::ops::RangeInclusive::new` in `synthyrion_smp::log2_q16`
loop `synthyrion_smp::log2_q16` header bb20
callee `<Vec<Cpu> as PartialEq>::eq` in `synthyrion_smp::madt::eq`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_smp::madt::fmt`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_smp::madt::fmt`
callee `Formatter::write_str` in `synthyrion_smp::madt::fmt`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_smp::mce::fmt`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_smp::mce::fmt`
callee `<Vec<u64> as Deref>::deref` in `synthyrion_smp::measure`
callee `<Vec<u8> as Deref>::deref` in `synthyrion_smp::measure`
callee `<core::ops::RangeInclusive<u32> as IntoIterator>::into_iter` in `synthyrion_smp::measured_colour_range`
callee `<core::ops::RangeInclusive<u32> as Iterator>::next` in `synthyrion_smp::measured_colour_range`
callee `<core::slice::Iter<'_, BitProbe> as Iterator>::find` in `synthyrion_smp::measured_colour_range`
callee `core::ops::RangeInclusive::new` in `synthyrion_smp::measured_colour_range`
loop `synthyrion_smp::measured_colour_range` header bb5
callee `<Enumerate<core::slice::Iter<'_, u64>> as IntoIterator>::into_iter` in `synthyrion_smp::mutual_information`
callee `<Enumerate<core::slice::Iter<'_, u64>> as Iterator>::next` in `synthyrion_smp::mutual_information`
callee `<[u64] as Index<RangeTo<usize>>>::index` in `synthyrion_smp::mutual_information`
callee `<core::slice::Iter<'_, u64> as Iterator>::enumerate` in `synthyrion_smp::mutual_information`
callee `<core::slice::Iter<'_, u64> as Iterator>::fold` in `synthyrion_smp::mutual_information`
callee `<core::slice::Iter<'_, u64> as Iterator>::sum` in `synthyrion_smp::mutual_information`
callee `<usize as Ord>::min` in `synthyrion_smp::mutual_information`
loop `synthyrion_smp::mutual_information` header bb10
loop `synthyrion_smp::mutual_information` header bb39
callee `<Rev<core::ops::Range<usize>> as IntoIterator>::into_iter` in `synthyrion_smp::noise_floor`
callee `<Rev<core::ops::Range<usize>> as Iterator>::next` in `synthyrion_smp::noise_floor`
callee `<Vec<u64> as Deref>::deref` in `synthyrion_smp::noise_floor`
callee `<Vec<u64> as DerefMut>::deref_mut` in `synthyrion_smp::noise_floor`
callee `<[u64] as Index<RangeTo<usize>>>::index` in `synthyrion_smp::noise_floor`
callee `<[u8] as Index<RangeTo<usize>>>::index` in `synthyrion_smp::noise_floor`
callee `<core::ops::Range<usize> as Iterator>::rev` in `synthyrion_smp::noise_floor`
callee `<usize as Ord>::min` in `synthyrion_smp::noise_floor`
callee `alloc::slice::to_vec` in `synthyrion_smp::noise_floor`
callee `core::slice::swap` in `synthyrion_smp::noise_floor`
loop `synthyrion_smp::noise_floor` header bb5
callee `<[&str; 7] as IntoIterator>::into_iter` in `synthyrion_smp::open_line`
callee `<core::array::IntoIter<&str, 7> as Iterator>::next` in `synthyrion_smp::open_line`
callee `Arguments::new` in `synthyrion_smp::open_line`
callee `String::push` in `synthyrion_smp::open_line`
callee `String::push_str` in `synthyrion_smp::open_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::open_line`
callee `format` in `synthyrion_smp::open_line`
callee `must_use` in `synthyrion_smp::open_line`
loop `synthyrion_smp::open_line` header bb6
callee `Arguments::new` in `synthyrion_smp::padres_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::padres_line`
callee `format` in `synthyrion_smp::padres_line`
callee `must_use` in `synthyrion_smp::padres_line`
callee `<&[u8] as PartialEq<&[u8; 4]>>::ne` in `synthyrion_smp::parse_madt`
callee `<Result<Madt, MadtError> as FromResidual<Result<Infallible, MadtError>>>::from_residual` in `synthyrion_smp::parse_madt`
callee `<Result<u32, MadtError> as Try>::branch` in `synthyrion_smp::parse_madt`
callee `<Result<u64, MadtError> as Try>::branch` in `synthyrion_smp::parse_madt`
callee `<[u8] as Index<RangeTo<usize>>>::index` in `synthyrion_smp::parse_madt`
callee `<[u8] as Index<core::ops::Range<usize>>>::index` in `synthyrion_smp::parse_madt`
callee `<core::slice::Iter<'_, u8> as Iterator>::fold` in `synthyrion_smp::parse_madt`
callee `Vec::len` in `synthyrion_smp::parse_madt`
callee `Vec::new` in `synthyrion_smp::parse_madt`
callee `Vec::push` in `synthyrion_smp::parse_madt`
loop `synthyrion_smp::parse_madt` header bb27
callee `<Enumerate<core::slice::Iter<'_, u64>> as Iterator>::take` in `synthyrion_smp::probe_candidates`
callee `<Take<Enumerate<core::slice::Iter<'_, u64>>> as IntoIterator>::into_iter` in `synthyrion_smp::probe_candidates`
callee `<Take<Enumerate<core::slice::Iter<'_, u64>>> as Iterator>::next` in `synthyrion_smp::probe_candidates`
callee `<core::slice::Iter<'_, u64> as Iterator>::enumerate` in `synthyrion_smp::probe_candidates`
callee `Vec::new` in `synthyrion_smp::probe_candidates`
callee `Vec::push` in `synthyrion_smp::probe_candidates`
loop `synthyrion_smp::probe_candidates` header bb11
callee `Formatter::write_str` in `synthyrion_smp::pvyield::fmt`
callee `Arguments::new` in `synthyrion_smp::r2_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::r2_line`
callee `format` in `synthyrion_smp::r2_line`
callee `must_use` in `synthyrion_smp::r2_line`
callee `Arguments::new` in `synthyrion_smp::r3_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::r3_line`
callee `core::fmt::rt::Argument::new_lower_hex` in `synthyrion_smp::r3_line`
callee `format` in `synthyrion_smp::r3_line`
callee `must_use` in `synthyrion_smp::r3_line`
callee `Arguments::new` in `synthyrion_smp::r4_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::r4_line`
callee `format` in `synthyrion_smp::r4_line`
callee `must_use` in `synthyrion_smp::r4_line`
callee `Arguments::new` in `synthyrion_smp::result_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::result_line`
callee `format` in `synthyrion_smp::result_line`
callee `must_use` in `synthyrion_smp::result_line`
callee `<&Vec<u64> as IntoIterator>::into_iter` in `synthyrion_smp::run_model`
callee `<FlatMap<core::slice::Iter<'_, u64>, Map<core::ops::Range<u64>, {closure@crates\synthyrion_smp\src\leakage.rs:443:75: 443:83}>, {closure@crates\synthyrion_smp\src\leakage.rs:443:56: 443:59}> as Iterator>::collect` in `synthyrion_smp::run_model`
callee `<FlatMap<core::slice::Iter<'_, u64>, Map<core::ops::Range<u64>, {closure@crates\synthyrion_smp\src\leakage.rs:451:71: 451:79}>, {closure@crates\synthyrion_smp\src\leakage.rs:451:52: 451:55}> as Iterator>::collect` in `synthyrion_smp::run_model`
callee `<FlatMap<core::slice::Iter<'_, u64>, Map<core::ops::Range<u64>, {closure@crates\synthyrion_smp\src\leakage.rs:458:54: 458:62}>, {closure@crates\synthyrion_smp\src\leakage.rs:458:35: 458:38}> as Iterator>::take` in `synthyrion_smp::run_model`
callee `<Map<core::ops::Range<u64>, {closure@crates\synthyrion_smp\src\leakage.rs:455:56: 455:59}> as Iterator>::collect` in `synthyrion_smp::run_model`
callee `<Map<core::slice::Iter<'_, u64>, {closure@crates\synthyrion_smp\src\leakage.rs:495:45: 495:49}> as Iterator>::sum` in `synthyrion_smp::run_model`
callee `<Take<FlatMap<core::slice::Iter<'_, u64>, Map<core::ops::Range<u64>, {closure@crates\synthyrion_smp\src\leakage.rs:458:54: 458:62}>, {closure@crates\synthyrion_smp\src\leakage.rs:458:35: 458:38}>> as Iterator>::collect` in `synthyrion_smp::run_model`
callee `<Vec<u64> as Deref>::deref` in `synthyrion_smp::run_model`
callee `<Vec<u64> as Extend<u64>>::extend` in `synthyrion_smp::run_model`
callee `<core::ops::Range<u64> as Iterator>::map` in `synthyrion_smp::run_model`
callee `<core::slice::Iter<'_, u64> as Iterator>::flat_map` in `synthyrion_smp::run_model`
callee `<core::slice::Iter<'_, u64> as Iterator>::map` in `synthyrion_smp::run_model`
callee `Arguments::from_str` in `synthyrion_smp::run_model`
callee `Vec::new` in `synthyrion_smp::run_model`
callee `Vec::push` in `synthyrion_smp::run_model`
callee `core::num::div_ceil` in `synthyrion_smp::run_model`
callee `core::num::rotate_left` in `synthyrion_smp::run_model`
callee `panic_fmt` in `synthyrion_smp::run_model`
loop `synthyrion_smp::run_model` header bb70
callee `<core::ops::Range<u64> as Iterator>::map` in `synthyrion_smp::run_model::{closure#1}`
callee `<&u64 as Add<u64>>::add` in `synthyrion_smp::run_model::{closure#1}::{closure#0}`
callee `<core::ops::Range<u64> as Iterator>::map` in `synthyrion_smp::run_model::{closure#2}`
callee `<&u64 as Add<u64>>::add` in `synthyrion_smp::run_model::{closure#2}::{closure#0}`
callee `<core::ops::Range<u64> as Iterator>::map` in `synthyrion_smp::run_model::{closure#4}`
callee `<&u64 as Add<u64>>::add` in `synthyrion_smp::run_model::{closure#4}::{closure#0}`
callee `Formatter::write_str` in `synthyrion_smp::shootdown::fmt`
callee `<core::ops::Range<u32> as IntoIterator>::into_iter` in `synthyrion_smp::siblings_of`
callee `<core::ops::Range<u32> as Iterator>::next` in `synthyrion_smp::siblings_of`
callee `<u32 as Ord>::min` in `synthyrion_smp::siblings_of`
callee `core::slice::get_mut` in `synthyrion_smp::siblings_of`
loop `synthyrion_smp::siblings_of` header bb9
callee `<Option<&str> as Try>::branch` in `synthyrion_smp::slot_of`
callee `<Option<(&str, &str)> as Try>::branch` in `synthyrion_smp::slot_of`
callee `<SplitWhitespace<'_> as Iterator>::next` in `synthyrion_smp::slot_of`
callee `<SplitWhitespace<'_> as Iterator>::nth` in `synthyrion_smp::slot_of`
callee `<core::slice::Iter<'_, &str> as Iterator>::position` in `synthyrion_smp::slot_of`
callee `<str as PartialEq>::eq` in `synthyrion_smp::slot_of`
callee `core::str::contains` in `synthyrion_smp::slot_of`
callee `core::str::parse` in `synthyrion_smp::slot_of`
callee `core::str::split_once` in `synthyrion_smp::slot_of`
callee `core::str::split_whitespace` in `synthyrion_smp::slot_of`
callee `core::str::strip_prefix` in `synthyrion_smp::slot_of`
callee `<&str as PartialEq>::eq` in `synthyrion_smp::slot_of::{closure#0}`
callee `<&str as PartialEq>::eq` in `synthyrion_smp::slot_of::{closure#2}`
callee `<impl MachineClock as Clock>::now` in `synthyrion_smp::switch_padded_with`
callee `<impl MachineClock as Clock>::spin_until` in `synthyrion_smp::switch_padded_with`
callee `<Copied<core::slice::Iter<'_, u64>> as Iterator>::max` in `synthyrion_smp::switchpad::SwitchPad::from_samples`
callee `<core::slice::Iter<'_, u64> as Iterator>::copied` in `synthyrion_smp::switchpad::SwitchPad::from_samples`
callee `core::num::div_ceil` in `synthyrion_smp::switchpad::SwitchPad::from_samples`
callee `<FlushDone as PartialEq>::eq` in `synthyrion_smp::switchpad::eq`
callee `Formatter::debug_struct_field1_finish` in `synthyrion_smp::switchpad::fmt`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_smp::switchpad::fmt`
callee `Formatter::write_str` in `synthyrion_smp::switchpad::fmt`
callee `<M as FlushMachine>::l1d_sweep` in `synthyrion_smp::switchpad::l1d_sweep`
callee `<M as FlushMachine>::l2_sweep` in `synthyrion_smp::switchpad::l2_sweep`
callee `<M as FlushMachine>::llc_wbinvd` in `synthyrion_smp::switchpad::llc_wbinvd`
callee `<C as Clock>::now` in `synthyrion_smp::switchpad::now`
callee `<C as Clock>::spin_until` in `synthyrion_smp::switchpad::spin_until`
callee `<M as FlushMachine>::tlb_flush` in `synthyrion_smp::switchpad::tlb_flush`
callee `<M as FlushMachine>::verw_md_clear` in `synthyrion_smp::switchpad::verw_md_clear`
callee `<M as FlushMachine>::wrmsr` in `synthyrion_smp::switchpad::wrmsr`
callee `core::num::next_power_of_two` in `synthyrion_smp::timeprot::SmtTopology::ids_per_core`
callee `<bool as Default>::default` in `synthyrion_smp::timeprot::default`
callee `<&u64 as PartialEq>::eq` in `synthyrion_smp::timeprot::eq`
callee `<&usize as PartialEq>::eq` in `synthyrion_smp::timeprot::eq`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_smp::timeprot::fmt`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_smp::timeprot::fmt`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_smp::timeprot::fmt`
callee `Formatter::write_str` in `synthyrion_smp::timeprot::fmt`
callee `<String as Clone>::clone` in `synthyrion_smp::tpreport::clone`
callee `<&str as PartialEq>::eq` in `synthyrion_smp::tpreport::eq`
callee `<Option<L2Facts> as PartialEq>::eq` in `synthyrion_smp::tpreport::eq`
callee `<Option<LlcFacts> as PartialEq>::eq` in `synthyrion_smp::tpreport::eq`
callee `<String as PartialEq>::eq` in `synthyrion_smp::tpreport::eq`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_smp::tpreport::fmt`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_smp::tpreport::fmt`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_smp::tpreport::fmt`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_smp::tpreport::fmt`
callee `<Q16 as PartialOrd>::gt` in `synthyrion_smp::verdict_at`
callee `Arguments::new` in `synthyrion_smp::verdict_line`
callee `core::fmt::rt::Argument::new_display` in `synthyrion_smp::verdict_line`
callee `format` in `synthyrion_smp::verdict_line`
callee `must_use` in `synthyrion_smp::verdict_line`
callee `String::new` in `synthyrion_smp::verdict_of`
callee `Vec::is_empty` in `synthyrion_smp::verdict_of`
callee `Vec::new` in `synthyrion_smp::verdict_of`
callee `Vec::push` in `synthyrion_smp::verdict_of`
callee `<Enumerate<core::slice::Iter<'_, &str>> as IntoIterator>::into_iter` in `synthyrion_smp::verdict_of::{closure#1}`
callee `<Enumerate<core::slice::Iter<'_, &str>> as Iterator>::next` in `synthyrion_smp::verdict_of::{closure#1}`
callee `<Vec<&str> as Deref>::deref` in `synthyrion_smp::verdict_of::{closure#1}`
callee `<core::slice::Iter<'_, &str> as Iterator>::enumerate` in `synthyrion_smp::verdict_of::{closure#1}`
callee `String::new` in `synthyrion_smp::verdict_of::{closure#1}`
callee `String::push` in `synthyrion_smp::verdict_of::{closure#1}`
callee `String::push_str` in `synthyrion_smp::verdict_of::{closure#1}`
loop `synthyrion_smp::verdict_of::{closure#1}` header bb6
callee `<EntryDone as Default>::default` in `synthyrion_smp::vm_entry_mitigate`
callee `<impl VmMachine as SpecMachine>::wrmsr` in `synthyrion_smp::vm_entry_mitigate`
callee `<impl VmMachine as VmMachine>::l1d_sweep` in `synthyrion_smp::vm_entry_mitigate`
callee `<impl VmMachine as VmMachine>::verw_md_clear` in `synthyrion_smp::vm_entry_mitigate`
callee `<ExitDone as Default>::default` in `synthyrion_smp::vm_exit_barrier`
callee `<impl VmMachine as SpecMachine>::wrmsr` in `synthyrion_smp::vm_exit_barrier`
callee `<impl VmMachine as VmMachine>::lfence` in `synthyrion_smp::vm_exit_barrier`
callee `<ExitFirstDone as Default>::default` in `synthyrion_smp::vm_exit_first`
callee `<impl SpecMachine as SpecMachine>::rsb_fill` in `synthyrion_smp::vm_exit_first`
callee `<impl SpecMachine as SpecMachine>::wrmsr` in `synthyrion_smp::vm_exit_first`
callee `<&[(u32, u64)] as IntoIterator>::into_iter` in `synthyrion_smp::vm_smt_gate`
callee `<core::slice::Iter<'_, (u32, u64)> as Iterator>::next` in `synthyrion_smp::vm_smt_gate`
callee `core::slice::get` in `synthyrion_smp::vm_smt_gate`
callee `core::slice::get_mut` in `synthyrion_smp::vm_smt_gate`
loop `synthyrion_smp::vm_smt_gate` header bb8
callee `<bool as Default>::default` in `synthyrion_smp::vmentry::default`
callee `<ArchCap as PartialEq>::eq` in `synthyrion_smp::vmentry::eq`
callee `<FlushPlan as PartialEq>::eq` in `synthyrion_smp::vmentry::eq`
callee `<SpecMode as PartialEq>::eq` in `synthyrion_smp::vmentry::eq`
callee `<[Option<&str>; 5] as PartialEq>::eq` in `synthyrion_smp::vmentry::eq`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_smp::vmentry::fmt`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_smp::vmentry::fmt`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_smp::vmentry::fmt`
callee `Formatter::debug_struct_field5_finish` in `synthyrion_smp::vmentry::fmt`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_smp::vmentry::fmt`
callee `Formatter::write_str` in `synthyrion_smp::vmentry::fmt`
callee `<[(u32, u32); 4] as IntoIterator>::into_iter` in `synthyrion_smp::walk4`
callee `<core::array::IntoIter<(u32, u32), 4> as Iterator>::next` in `synthyrion_smp::walk4`
callee `<impl FnMut(u64) -> u64 as FnMut<(u64,)>>::call_mut` in `synthyrion_smp::walk4`
loop `synthyrion_smp::walk4` header bb6
callee `<Cell<bool> as Clone>::clone` in `synthyrion_vmm::<AddressSpace as Clone>::clone`
callee `<Cell<u64> as Clone>::clone` in `synthyrion_vmm::<AddressSpace as Clone>::clone`
callee `<S as Clone>::clone` in `synthyrion_vmm::<AddressSpace as Clone>::clone`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_vmm::<AddressSpace as Debug>::fmt`
callee `<Option<Link> as Try>::branch` in `synthyrion_vmm::<Chain as Iterator>::next`
callee `<Cell<u64> as Clone>::clone` in `synthyrion_vmm::<ChargedTables as Clone>::clone`
callee `<M as Clone>::clone` in `synthyrion_vmm::<ChargedTables as Clone>::clone`
callee `<ManuallyDrop<Vec<(u64, u64, u64)>> as Clone>::clone` in `synthyrion_vmm::<ChargedTables as Clone>::clone`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_vmm::<ChargedTables as Debug>::fmt`
callee `<Cell<u64> as Default>::default` in `synthyrion_vmm::<ChargedTables as Default>::default`
callee `<M as Default>::default` in `synthyrion_vmm::<ChargedTables as Default>::default`
callee `<ManuallyDrop<Vec<(u64, u64, u64)>> as Default>::default` in `synthyrion_vmm::<ChargedTables as Default>::default`
callee `<[u64; 16] as Default>::default` in `synthyrion_vmm::<ChargedTables as Default>::default`
callee `<bool as Default>::default` in `synthyrion_vmm::<ChargedTables as Default>::default`
callee `<u64 as Default>::default` in `synthyrion_vmm::<ChargedTables as Default>::default`
callee `<usize as Default>::default` in `synthyrion_vmm::<ChargedTables as Default>::default`
callee `<Vec<u64> as DerefMut>::deref_mut` in `synthyrion_vmm::<ChargedTables as TableStore>::bases`
callee `core::slice::sort_unstable` in `synthyrion_vmm::<ChargedTables as TableStore>::bases`
callee `<M as TableMem>::can_provide` in `synthyrion_vmm::<ChargedTables as TableStore>::can_provide`
callee `<ChargedTables<M> as TableStore>::contains` in `synthyrion_vmm::<ChargedTables as TableStore>::get`
callee `<M as TableMem>::tables` in `synthyrion_vmm::<ChargedTables as TableStore>::get`
callee `<Option<(&[u64], &[u32])> as Try>::branch` in `synthyrion_vmm::<ChargedTables as TableStore>::get`
callee `<Option<TableRef<'_>> as FromResidual<Option<Infallible>>>::from_residual` in `synthyrion_vmm::<ChargedTables as TableStore>::get`
callee `GuardedRef::open` in `synthyrion_vmm::<ChargedTables as TableStore>::get`
callee `<ChargedTables<M> as TableStore>::contains` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `<M as TableMem>::tables_mut` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `<M as TableMem>::tables_mut_map` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `<ManuallyDrop<Vec<(u64, u64, u64)>> as Deref>::deref` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `<Option<(&mut [u64], &mut [u32])> as Try>::branch` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `<Option<(&mut [u64], &mut [u32], &[(u64, u64, u64)])> as Try>::branch` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `<Option<TableMut<'_>> as FromResidual<Option<Infallible>>>::from_residual` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `<Vec<(u64, u64, u64)> as Deref>::deref` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `GuardedView::open` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `Vec::is_empty` in `synthyrion_vmm::<ChargedTables as TableStore>::get_mut`
callee `<M as TableMem>::tables_mut` in `synthyrion_vmm::<ChargedTables as TableStore>::insert_new`
callee `GuardedView::format` in `synthyrion_vmm::<ChargedTables as TableStore>::insert_new`
callee `Cell::get` in `synthyrion_vmm::<ChargedTables as TableStore>::link_faults`
callee `<Option<Link> as Try>::branch` in `synthyrion_vmm::<ChargedTables as TableStore>::next_table`
callee `<Filter<core::slice::Iter<'_, u64>, {closure@crates\synthyrion_vmm\src\lib.rs:852:40: 852:43}> as Iterator>::count` in `synthyrion_vmm::<ChargedTables as TableStore>::pair_count`
callee `<Filter<core::slice::Iter<'_, u64>, {closure@crates\synthyrion_vmm\src\lib.rs:855:51: 855:54}> as Iterator>::count` in `synthyrion_vmm::<ChargedTables as TableStore>::pair_count`
callee `<Vec<u64> as Deref>::deref` in `synthyrion_vmm::<ChargedTables as TableStore>::pair_count`
callee `<Vec<u64> as Extend<u64>>::extend` in `synthyrion_vmm::<ChargedTables as TableStore>::pair_count`
callee `<core::slice::Iter<'_, u64> as Iterator>::filter` in `synthyrion_vmm::<ChargedTables as TableStore>::pair_count`
callee `<M as TableMem>::provide` in `synthyrion_vmm::<ChargedTables as TableStore>::provide`
callee `<M as TableMem>::tables` in `synthyrion_vmm::<ChargedTables as TableStore>::refuses`
callee `<ChargedTables<M> as TableStore>::sd_frames` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `<Link as Default>::default` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `<Vec<u64> as Deref>::deref` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `<Vec<u64> as DerefMut>::deref_mut` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `<Vec<u64> as Extend<u64>>::extend` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `<core::iter::Chain<core::slice::Iter<'_, u64>, core::slice::Iter<'_, u64>> as IntoIterator>::into_iter` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `<core::iter::Chain<core::slice::Iter<'_, u64>, core::slice::Iter<'_, u64>> as Iterator>::next` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `Vec::len` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `Vec::new` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `Vec::push` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
callee `core::slice::reverse` in `synthyrion_vmm::<ChargedTables as TableStore>::release_links`
loop `synthyrion_vmm::<ChargedTables as TableStore>::release_links` header bb11
loop `synthyrion_vmm::<ChargedTables as TableStore>::release_links` header bb24
loop `synthyrion_vmm::<ChargedTables as TableStore>::release_links` header bb39
callee `<Vec<u64> as DerefMut>::deref_mut` in `synthyrion_vmm::<ChargedTables as TableStore>::sd_frames`
callee `core::slice::reverse` in `synthyrion_vmm::<ChargedTables as TableStore>::sd_frames`
callee `<Option<Link> as Try>::branch` in `synthyrion_vmm::<ChargedTables as TableStore>::sd_pop`
callee `<Option<Link> as Try>::branch` in `synthyrion_vmm::<ChargedTables as TableStore>::sup_pop`
callee `Cell::get` in `synthyrion_vmm::<ChargedTables as TableStore>::verify_chains`
callee `Formatter::debug_struct_field5_finish` in `synthyrion_vmm::<EffectivePerms as Debug>::fmt`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_vmm::<IntegrityStats as Debug>::fmt`
callee `<u64 as Default>::default` in `synthyrion_vmm::<IntegrityStats as Default>::default`
callee `Formatter::write_str` in `synthyrion_vmm::<MapError as Debug>::fmt`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_vmm::<Occupancy as Debug>::fmt`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_vmm::<Occupancy as Debug>::fmt`
callee `Formatter::write_str` in `synthyrion_vmm::<Occupancy as Debug>::fmt`
callee `<&PageSize as PartialEq>::eq` in `synthyrion_vmm::<Occupancy as PartialEq>::eq`
callee `<&u64 as PartialEq>::eq` in `synthyrion_vmm::<Occupancy as PartialEq>::eq`
callee `<&usize as PartialEq>::eq` in `synthyrion_vmm::<Occupancy as PartialEq>::eq`
callee `<ChargedTables<M> as Clone>::clone` in `synthyrion_vmm::<TableBackend as Clone>::clone`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_vmm::<TableBackend as Debug>::fmt`
callee `<M as Default>::default` in `synthyrion_vmm::<TableBackend as Default>::default`
callee `<ChargedTables<M> as TableStore>::bases` in `synthyrion_vmm::<TableBackend as TableStore>::bases`
callee `<ChargedTables<M> as TableStore>::can_provide` in `synthyrion_vmm::<TableBackend as TableStore>::can_provide`
callee `<ChargedTables<M> as TableStore>::contains` in `synthyrion_vmm::<TableBackend as TableStore>::contains`
callee `<ChargedTables<M> as TableStore>::first_table` in `synthyrion_vmm::<TableBackend as TableStore>::first_table`
callee `<ChargedTables<M> as TableStore>::get` in `synthyrion_vmm::<TableBackend as TableStore>::get`
callee `<ChargedTables<M> as TableStore>::get_mut` in `synthyrion_vmm::<TableBackend as TableStore>::get_mut`
callee `<ChargedTables<M> as TableStore>::heap_pages` in `synthyrion_vmm::<TableBackend as TableStore>::heap_pages`
callee `<ChargedTables<M> as TableStore>::insert_new` in `synthyrion_vmm::<TableBackend as TableStore>::insert_new`
callee `<ChargedTables<M> as TableStore>::len` in `synthyrion_vmm::<TableBackend as TableStore>::len`
callee `<ChargedTables<M> as TableStore>::link_faults` in `synthyrion_vmm::<TableBackend as TableStore>::link_faults`
callee `<ChargedTables<M> as TableStore>::next_table` in `synthyrion_vmm::<TableBackend as TableStore>::next_table`
callee `<ChargedTables<M> as TableStore>::pair_count` in `synthyrion_vmm::<TableBackend as TableStore>::pair_count`
callee `<ChargedTables<M> as TableStore>::provide` in `synthyrion_vmm::<TableBackend as TableStore>::provide`
callee `<ChargedTables<M> as TableStore>::refuses` in `synthyrion_vmm::<TableBackend as TableStore>::refuses`
callee `<ChargedTables<M> as TableStore>::release_links` in `synthyrion_vmm::<TableBackend as TableStore>::release_links`
callee `<ChargedTables<M> as TableStore>::sd_contains` in `synthyrion_vmm::<TableBackend as TableStore>::sd_contains`
callee `<ChargedTables<M> as TableStore>::sd_frames` in `synthyrion_vmm::<TableBackend as TableStore>::sd_frames`
callee `<ChargedTables<M> as TableStore>::sd_len` in `synthyrion_vmm::<TableBackend as TableStore>::sd_len`
callee `<ChargedTables<M> as TableStore>::sd_pop` in `synthyrion_vmm::<TableBackend as TableStore>::sd_pop`
callee `<ChargedTables<M> as TableStore>::sd_push` in `synthyrion_vmm::<TableBackend as TableStore>::sd_push`
callee `<ChargedTables<M> as TableStore>::set_owner` in `synthyrion_vmm::<TableBackend as TableStore>::set_owner`
callee `<ChargedTables<M> as TableStore>::sup_contains` in `synthyrion_vmm::<TableBackend as TableStore>::sup_contains`
callee `<ChargedTables<M> as TableStore>::sup_len` in `synthyrion_vmm::<TableBackend as TableStore>::sup_len`
callee `<ChargedTables<M> as TableStore>::sup_pop` in `synthyrion_vmm::<TableBackend as TableStore>::sup_pop`
callee `<ChargedTables<M> as TableStore>::sup_push` in `synthyrion_vmm::<TableBackend as TableStore>::sup_push`
callee `<ChargedTables<M> as TableStore>::verify_chains` in `synthyrion_vmm::<TableBackend as TableStore>::verify_chains`
callee `<ChargedTables<M> as TableStore>::window_frames` in `synthyrion_vmm::<TableBackend as TableStore>::window_frames`
callee `<synthyrion_kheal::Scan<'_, 18446744073709551519> as Iterator>::next` in `synthyrion_vmm::<TableScan as Iterator>::next`
callee `<S as TableStore>::sup_push` in `synthyrion_vmm::AddressSpace::add_table_frames`
loop `synthyrion_vmm::AddressSpace::add_table_frames` header bb4
callee `<S as TableStore>::insert_new` in `synthyrion_vmm::AddressSpace::alloc_table`
callee `<S as TableStore>::sd_pop` in `synthyrion_vmm::AddressSpace::alloc_table`
callee `<S as TableStore>::sd_push` in `synthyrion_vmm::AddressSpace::alloc_table`
callee `<S as TableStore>::sup_pop` in `synthyrion_vmm::AddressSpace::alloc_table`
callee `<S as TableStore>::sup_push` in `synthyrion_vmm::AddressSpace::alloc_table`
callee `<S as TableStore>::can_provide` in `synthyrion_vmm::AddressSpace::can_provide_table_frame`
callee `<PageSize as PartialEq>::eq` in `synthyrion_vmm::AddressSpace::child_leaf_pa`
callee `<S as TableStore>::release_links` in `synthyrion_vmm::AddressSpace::drain_table_frames`
callee `<S as TableStore>::release_links` in `synthyrion_vmm::AddressSpace::drain_table_frames_with_pairs`
callee `<S as TableStore>::get` in `synthyrion_vmm::AddressSpace::for_each_leaf`
callee `<TableScan<'_> as IntoIterator>::into_iter` in `synthyrion_vmm::AddressSpace::for_each_leaf`
callee `<TableScan<'_> as Iterator>::next` in `synthyrion_vmm::AddressSpace::for_each_leaf`
callee `<impl FnMut(u64, u64, bigpage::PageSize, u64) as FnMut<(u64, u64, PageSize, u64)>>::call_mut` in `synthyrion_vmm::AddressSpace::for_each_leaf`
loop `synthyrion_vmm::AddressSpace::for_each_leaf` header bb5
callee `<Option<TableRef<'_>> as Try>::branch` in `synthyrion_vmm::AddressSpace::hardware_word`
callee `<S as TableStore>::get` in `synthyrion_vmm::AddressSpace::hardware_word`
callee `GuardedRef::raw` in `synthyrion_vmm::AddressSpace::hardware_word`
callee `Cell::set` in `synthyrion_vmm::AddressSpace::heal`
callee `<PageSize as PartialEq>::eq` in `synthyrion_vmm::AddressSpace::heal_path`
loop `synthyrion_vmm::AddressSpace::heal_table` header bb2
callee `<S as TableStore>::heap_pages` in `synthyrion_vmm::AddressSpace::heap_table_pages`
callee `<S as TableStore>::contains` in `synthyrion_vmm::AddressSpace::holds_table_frame`
callee `<S as TableStore>::sd_contains` in `synthyrion_vmm::AddressSpace::holds_table_frame`
callee `<S as TableStore>::sup_contains` in `synthyrion_vmm::AddressSpace::holds_table_frame`
callee `Cell::get` in `synthyrion_vmm::AddressSpace::integrity`
callee `<S as TableStore>::link_faults` in `synthyrion_vmm::AddressSpace::is_poisoned`
callee `<S as TableStore>::get` in `synthyrion_vmm::AddressSpace::leaf_frames`
callee `<TableScan<'_> as IntoIterator>::into_iter` in `synthyrion_vmm::AddressSpace::leaf_frames`
callee `<TableScan<'_> as Iterator>::next` in `synthyrion_vmm::AddressSpace::leaf_frames`
callee `<Vec<u64> as DerefMut>::deref_mut` in `synthyrion_vmm::AddressSpace::leaf_frames`
callee `Vec::dedup` in `synthyrion_vmm::AddressSpace::leaf_frames`
callee `Vec::new` in `synthyrion_vmm::AddressSpace::leaf_frames`
callee `Vec::push` in `synthyrion_vmm::AddressSpace::leaf_frames`
callee `core::slice::sort_unstable` in `synthyrion_vmm::AddressSpace::leaf_frames`
loop `synthyrion_vmm::AddressSpace::leaf_frames` header bb7
callee `<S as TableStore>::contains` in `synthyrion_vmm::AddressSpace::map`
callee `<S as TableStore>::sup_len` in `synthyrion_vmm::AddressSpace::map`
callee `<S as TableStore>::sup_len` in `synthyrion_vmm::AddressSpace::map_large`
callee `Cell::get` in `synthyrion_vmm::AddressSpace::needs_heal`
callee `<TableBackend<M> as TableStore>::insert_new` in `synthyrion_vmm::AddressSpace::new_charged`
callee `<TableBackend<M> as TableStore>::set_owner` in `synthyrion_vmm::AddressSpace::new_charged`
callee `<TableBackend<M> as TableStore>::insert_new` in `synthyrion_vmm::AddressSpace::new_charged_mapped`
callee `<TableBackend<M> as TableStore>::set_owner` in `synthyrion_vmm::AddressSpace::new_charged_mapped`
callee `<S as TableStore>::insert_new` in `synthyrion_vmm::AddressSpace::new_in`
callee `<S as TableStore>::set_owner` in `synthyrion_vmm::AddressSpace::new_in`
callee `<S as TableStore>::insert_new` in `synthyrion_vmm::AddressSpace::new_lenient`
callee `<S as TableStore>::set_owner` in `synthyrion_vmm::AddressSpace::new_lenient`
callee `Cell::get` in `synthyrion_vmm::AddressSpace::note_fault`
callee `Cell::set` in `synthyrion_vmm::AddressSpace::note_fault`
loop `synthyrion_vmm::AddressSpace::present_children` header bb2
callee `<S as TableStore>::provide` in `synthyrion_vmm::AddressSpace::provide_table_frame`
callee `<Occupancy as PartialEq>::ne` in `synthyrion_vmm::AddressSpace::range_occupancy_healing`
callee `<S as TableStore>::get` in `synthyrion_vmm::AddressSpace::rd`
callee `Cell::get` in `synthyrion_vmm::AddressSpace::rd`
callee `Cell::set` in `synthyrion_vmm::AddressSpace::rd`
callee `<S as TableStore>::contains` in `synthyrion_vmm::AddressSpace::rd_heal`
callee `<S as TableStore>::get_mut` in `synthyrion_vmm::AddressSpace::rd_heal`
callee `<S as TableStore>::refuses` in `synthyrion_vmm::AddressSpace::rd_heal`
callee `<S as TableStore>::pair_count` in `synthyrion_vmm::AddressSpace::recorded_pair_count`
callee `<S as TableStore>::first_table` in `synthyrion_vmm::AddressSpace::scrub`
callee `<S as TableStore>::get_mut` in `synthyrion_vmm::AddressSpace::scrub`
callee `<S as TableStore>::len` in `synthyrion_vmm::AddressSpace::scrub`
callee `<S as TableStore>::next_table` in `synthyrion_vmm::AddressSpace::scrub`
callee `<ScrubStats as Default>::default` in `synthyrion_vmm::AddressSpace::scrub`
callee `<usize as Ord>::min` in `synthyrion_vmm::AddressSpace::scrub`
callee `Cell::set` in `synthyrion_vmm::AddressSpace::scrub`
callee `ScrubStats::add` in `synthyrion_vmm::AddressSpace::scrub`
loop `synthyrion_vmm::AddressSpace::scrub` header bb7
callee `<S as TableStore>::len` in `synthyrion_vmm::AddressSpace::scrub_slots`
callee `<S as TableStore>::contains` in `synthyrion_vmm::AddressSpace::set_hardware_entry`
callee `<S as TableStore>::get_mut` in `synthyrion_vmm::AddressSpace::set_hardware_entry`
callee `<S as TableStore>::refuses` in `synthyrion_vmm::AddressSpace::set_hardware_entry`
callee `<S as TableStore>::sd_frames` in `synthyrion_vmm::AddressSpace::supplied_table_frames`
callee `<S as TableStore>::bases` in `synthyrion_vmm::AddressSpace::table_frames`
callee `<S as TableStore>::len` in `synthyrion_vmm::AddressSpace::table_page_count`
callee `<FilterMap<core::ops::Range<usize>, {closure@crates\synthyrion_vmm\src\lib.rs:1546:34: 1546:37}> as Iterator>::collect` in `synthyrion_vmm::AddressSpace::table_present_entries`
callee `<S as TableStore>::contains` in `synthyrion_vmm::AddressSpace::table_present_entries`
callee `<core::ops::Range<usize> as Iterator>::filter_map` in `synthyrion_vmm::AddressSpace::table_present_entries`
callee `Vec::new` in `synthyrion_vmm::AddressSpace::table_present_entries`
callee `Cell::get` in `synthyrion_vmm::AddressSpace::table_reads`
callee `<S as TableStore>::sup_len` in `synthyrion_vmm::AddressSpace::table_supply_len`
callee `<[(u8, usize); 3] as IntoIterator>::into_iter` in `synthyrion_vmm::AddressSpace::tables_needed`
callee `<core::array::IntoIter<(u8, usize), 3> as Iterator>::next` in `synthyrion_vmm::AddressSpace::tables_needed`
loop `synthyrion_vmm::AddressSpace::tables_needed` header bb3
loop `synthyrion_vmm::AddressSpace::tables_needed_range` header bb9
callee `<S as TableStore>::sd_push` in `synthyrion_vmm::AddressSpace::take_spare_supply_frame`
callee `<S as TableStore>::sup_pop` in `synthyrion_vmm::AddressSpace::take_spare_supply_frame`
callee `<S as TableStore>::sup_push` in `synthyrion_vmm::AddressSpace::take_spare_supply_frame`
callee `<PageSize as PartialEq>::eq` in `synthyrion_vmm::AddressSpace::target`
callee `<S as TableStore>::len` in `synthyrion_vmm::AddressSpace::unsupplied_table_count`
callee `<S as TableStore>::sd_len` in `synthyrion_vmm::AddressSpace::unsupplied_table_count`
callee `<S as TableStore>::verify_chains` in `synthyrion_vmm::AddressSpace::verify_links`
callee `<S as TableStore>::window_frames` in `synthyrion_vmm::AddressSpace::window_table_frames`
callee `Cell::new` in `synthyrion_vmm::AddressSpace::with_root_table`
callee `<S as TableStore>::contains` in `synthyrion_vmm::AddressSpace::wr`
callee `<S as TableStore>::get_mut` in `synthyrion_vmm::AddressSpace::wr`
callee `<S as TableStore>::refuses` in `synthyrion_vmm::AddressSpace::wr`
callee `<S as TableStore>::contains` in `synthyrion_vmm::AddressSpace::wr_prechecked`
callee `<S as TableStore>::get_mut` in `synthyrion_vmm::AddressSpace::wr_prechecked`
callee `<S as TableStore>::refuses` in `synthyrion_vmm::AddressSpace::wr_prechecked`
callee `Cell::get` in `synthyrion_vmm::ChargedTables::chain_fault`
callee `Cell::set` in `synthyrion_vmm::ChargedTables::chain_fault`
callee `<M as TableMem>::tables_mut` in `synthyrion_vmm::ChargedTables::link_heal`
callee `<Option<(&mut [u64], &mut [u32])> as Try>::branch` in `synthyrion_vmm::ChargedTables::link_heal`
callee `<Option<Link> as FromResidual<Option<Infallible>>>::from_residual` in `synthyrion_vmm::ChargedTables::link_heal`
callee `Cell::get` in `synthyrion_vmm::ChargedTables::link_heal`
callee `Cell::set` in `synthyrion_vmm::ChargedTables::link_heal`
callee `<M as TableMem>::tables` in `synthyrion_vmm::ChargedTables::link_of`
callee `<Option<(&[u64], &[u32])> as Try>::branch` in `synthyrion_vmm::ChargedTables::link_of`
callee `<Option<Link> as FromResidual<Option<Infallible>>>::from_residual` in `synthyrion_vmm::ChargedTables::link_of`
callee `Cell::get` in `synthyrion_vmm::ChargedTables::link_of`
callee `Cell::set` in `synthyrion_vmm::ChargedTables::link_of`
callee `<M as TableMem>::tables_mut` in `synthyrion_vmm::ChargedTables::link_put`
callee `Cell::new` in `synthyrion_vmm::ChargedTables::new`
callee `ManuallyDrop::new` in `synthyrion_vmm::ChargedTables::new`
callee `Vec::new` in `synthyrion_vmm::ChargedTables::new`
callee `<M as TableMem>::entry_map` in `synthyrion_vmm::ChargedTables::region_map`
callee `<ManuallyDrop<Vec<(u64, u64, u64)>> as Deref>::deref` in `synthyrion_vmm::ChargedTables::region_map`
callee `<Vec<(u64, u64, u64)> as Deref>::deref` in `synthyrion_vmm::ChargedTables::region_map`
callee `Vec::is_empty` in `synthyrion_vmm::ChargedTables::region_map`
callee `<&mut Chain<'_, M> as Iterator>::collect` in `synthyrion_vmm::ChargedTables::walk`
callee `<Chain<'_, M> as Iterator>::by_ref` in `synthyrion_vmm::ChargedTables::walk`
callee `Vec::len` in `synthyrion_vmm::ChargedTables::walk`
callee `Cell::new` in `synthyrion_vmm::ChargedTables::with_map`
callee `ManuallyDrop::new` in `synthyrion_vmm::ChargedTables::with_map`
callee `<Self as TableMem>::tables` in `synthyrion_vmm::TableMem::pair`
callee `<Self as TableMem>::tables_mut` in `synthyrion_vmm::TableMem::pair_mut`
callee `<Option<(&mut [u64], &mut [u32])> as Try>::branch` in `synthyrion_vmm::TableMem::pair_mut_map`
callee `<Option<(&mut [u64], &mut [u32], &[(u64, u64, u64)])> as FromResidual<Option<Infallible>>>::from_residual` in `synthyrion_vmm::TableMem::pair_mut_map`
callee `<Self as TableMem>::pair_mut` in `synthyrion_vmm::TableMem::pair_mut_map`
callee `<Option<(&mut [u64], &mut [u32])> as Try>::branch` in `synthyrion_vmm::TableMem::tables_mut_map`
callee `<Option<(&mut [u64], &mut [u32], &[(u64, u64, u64)])> as FromResidual<Option<Infallible>>>::from_residual` in `synthyrion_vmm::TableMem::tables_mut_map`
callee `<Self as TableMem>::tables_mut` in `synthyrion_vmm::TableMem::tables_mut_map`
callee `GuardedView::heal_word` in `synthyrion_vmm::TableMut::heal_word`
callee `GuardedView::raw` in `synthyrion_vmm::TableMut::raw`
callee `GuardedView::scrub_range` in `synthyrion_vmm::TableMut::scrub_range`
callee `GuardedView::set_counted` in `synthyrion_vmm::TableMut::set_counted`
callee `GuardedView::set_counted` in `synthyrion_vmm::TableMut::set_hardware`
callee `GuardedView::set_prechecked` in `synthyrion_vmm::TableMut::set_prechecked`
callee `GuardedRef::get` in `synthyrion_vmm::TableRef::get`
callee `GuardedRef::raw` in `synthyrion_vmm::TableRef::raw`
callee `GuardedRef::scan` in `synthyrion_vmm::TableRef::scan`
callee `<Self as TableStore>::len` in `synthyrion_vmm::TableStore::is_empty`
callee `<&AsidError as PartialEq>::eq` in `synthyrion_vmm::armasid::eq`
callee `<&u16 as PartialEq>::eq` in `synthyrion_vmm::armasid::eq`
callee `<&u64 as PartialEq>::eq` in `synthyrion_vmm::armasid::eq`
callee `<Option<TlbOp> as PartialEq>::eq` in `synthyrion_vmm::armasid::eq`
callee `Formatter::debug_struct_field1_finish` in `synthyrion_vmm::armasid::fmt`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_vmm::armasid::fmt`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_vmm::armasid::fmt`
callee `Formatter::write_str` in `synthyrion_vmm::armasid::fmt`
callee `<&CacheInsn as PartialEq>::eq` in `synthyrion_vmm::armcache::eq`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_vmm::armcache::fmt`
callee `Formatter::write_str` in `synthyrion_vmm::armcache::fmt`
callee `<Result<SwitchKind, AsidError> as FromResidual<Result<Infallible, AsidError>>>::from_residual` in `synthyrion_vmm::asid::AsidPool::switch_to`
callee `<Result<u16, AsidError> as Try>::branch` in `synthyrion_vmm::asid::AsidPool::switch_to`
callee `<Result<u64, AsidError> as Try>::branch` in `synthyrion_vmm::asid::AsidPool::switch_to`
callee `<u16 as Default>::default` in `synthyrion_vmm::asid::default`
callee `<u64 as Default>::default` in `synthyrion_vmm::asid::default`
callee `Formatter::debug_struct_field2_finish` in `synthyrion_vmm::asid::fmt`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_vmm::asid::fmt`
callee `Formatter::write_str` in `synthyrion_vmm::asid::fmt`
callee `<PageSize as PartialEq>::eq` in `synthyrion_vmm::bigpage::eq`
callee `Formatter::debug_struct_fields_finish` in `synthyrion_vmm::bigpage::fmt`
callee `Formatter::write_str` in `synthyrion_vmm::bigpage::fmt`
callee `<&[u32] as IntoIterator>::into_iter` in `synthyrion_vmm::blank`
callee `core::slice::get` in `synthyrion_vmm::blank`
loop `synthyrion_vmm::blank` header bb5
callee `Formatter::debug_struct_field2_finish` in `synthyrion_vmm::cachemaint::fmt`
callee `Formatter::write_str` in `synthyrion_vmm::cachemaint::fmt`
callee `core::ops::Range::contains` in `synthyrion_vmm::canonical::{closure#0}`
callee `core::num::is_multiple_of` in `synthyrion_vmm::check_alone`
callee `<&[bigpage::Map] as IntoIterator>::into_iter` in `synthyrion_vmm::check_map`
callee `<Result<(), BigError> as FromResidual<Result<Infallible, BigError>>>::from_residual` in `synthyrion_vmm::check_map`
callee `<Result<(), BigError> as Try>::branch` in `synthyrion_vmm::check_map`
callee `<core::slice::Iter<'_, bigpage::Map> as Iterator>::next` in `synthyrion_vmm::check_map`
loop `synthyrion_vmm::check_map` header bb7
callee `<Result<(), BigError> as FromResidual<Result<Infallible, BigError>>>::from_residual` in `synthyrion_vmm::check_range`
callee `<Result<(), BigError> as Try>::branch` in `synthyrion_vmm::check_range`
callee `<Option<Link> as FromResidual<Option<Infallible>>>::from_residual` in `synthyrion_vmm::copy_ok`
callee `<Option<[u32; 12]> as Try>::branch` in `synthyrion_vmm::copy_ok`
loop `synthyrion_vmm::effective_perms` header bb10
callee `<Option<(u32, u32, u32)> as Try>::branch` in `synthyrion_vmm::field_ok`
callee `core::slice::get` in `synthyrion_vmm::fill`
loop `synthyrion_vmm::fill` header bb2
callee `<Result<FlushPlan, CacheError> as FromResidual<Result<Infallible, CacheError>>>::from_residual` in `synthyrion_vmm::flush_plan`
callee `<Result<u64, CacheError> as Try>::branch` in `synthyrion_vmm::flush_plan`
callee `<&[(u64, u64, u64)] as IntoIterator>::into_iter` in `synthyrion_vmm::from_stored`
loop `synthyrion_vmm::from_stored` header bb4
callee `core::slice::get_mut` in `synthyrion_vmm::put`
callee `<Zip<core::slice::IterMut<'_, u32>, core::slice::Iter<'_, u32>> as IntoIterator>::into_iter` in `synthyrion_vmm::put_copy`
callee `<Zip<core::slice::IterMut<'_, u32>, core::slice::Iter<'_, u32>> as Iterator>::next` in `synthyrion_vmm::put_copy`
callee `<core::slice::IterMut<'_, u32> as Iterator>::zip` in `synthyrion_vmm::put_copy`
callee `core::slice::get` in `synthyrion_vmm::put_copy`
callee `core::slice::get_mut` in `synthyrion_vmm::put_copy`
loop `synthyrion_vmm::put_copy` header bb25
callee `core::slice::get_mut` in `synthyrion_vmm::put_copy::{closure#0}`
callee `core::slice::get` in `synthyrion_vmm::put_field`
callee `core::slice::get_mut` in `synthyrion_vmm::put_field`
callee `<Result<TreeRead, TreeFault> as Try>::branch` in `synthyrion_vmm::read_all_heal`
callee `<Result<[u64; 16], TreeFault> as FromResidual<Result<Infallible, TreeFault>>>::from_residual` in `synthyrion_vmm::read_all_heal`
loop `synthyrion_vmm::read_all_heal` header bb2
callee `<Result<TreeRead, TreeFault> as Try>::branch` in `synthyrion_vmm::read_all_peek`
callee `<Result<[u64; 16], TreeFault> as FromResidual<Result<Infallible, TreeFault>>>::from_residual` in `synthyrion_vmm::read_all_peek`
loop `synthyrion_vmm::read_all_peek` header bb2
callee `<M as TableMem>::tables_mut` in `synthyrion_vmm::record_table_side`
callee `<AsidPool as Clone>::clone` in `synthyrion_vmm::switch`
callee `<Result<ArmSwitch, ArmError> as FromResidual<Result<Infallible, ArmError>>>::from_residual` in `synthyrion_vmm::switch`
callee `<Result<SwitchKind, ArmError> as Try>::branch` in `synthyrion_vmm::switch`
callee `Result::map_err ArmError {ArmError::Pool}>` in `synthyrion_vmm::switch`
callee `core::slice::get_mut` in `synthyrion_vmm::tablelink::corrupt`
callee `core::slice::get_mut` in `synthyrion_vmm::tablelink::corrupt::{closure#0}`
callee `<u64 as Default>::default` in `synthyrion_vmm::tablelink::default`
callee `<&Link as PartialEq>::eq` in `synthyrion_vmm::tablelink::eq`
callee `Formatter::debug_struct_field4_finish` in `synthyrion_vmm::tablelink::fmt`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_vmm::tablelink::fmt`
callee `Formatter::write_str` in `synthyrion_vmm::tablelink::fmt`
callee `<Link as Default>::default` in `synthyrion_vmm::tablelink::format`
callee `<Link as Default>::default` in `synthyrion_vmm::tablelink::peek`
callee `<Link as PartialEq>::eq` in `synthyrion_vmm::tablelink::peek`
callee `core::slice::get` in `synthyrion_vmm::tablelink::peek`
callee `<core::slice::Iter<'_, u32> as Iterator>::all` in `synthyrion_vmm::tablelink::peek::{closure#0}`
callee `<Result<LinkRead, LinkFault> as FromResidual<Result<Infallible, LinkFault>>>::from_residual` in `synthyrion_vmm::tablelink::read`
callee `<Result<LinkRead, LinkFault> as Try>::branch` in `synthyrion_vmm::tablelink::read`
callee `<Option<&[u64; 4]> as Try>::branch` in `synthyrion_vmm::tablelink::sum`
callee `check_word` in `synthyrion_vmm::tablelink::sum`
callee `core::slice::get` in `synthyrion_vmm::tablelink::sum`
callee `<&[u32] as TryInto<[u32; 12]>>::try_into` in `synthyrion_vmm::tablelink::words`
callee `<Option<&[u32]> as Try>::branch` in `synthyrion_vmm::tablelink::words`
callee `<Option<&usize> as Try>::branch` in `synthyrion_vmm::tablelink::words`
callee `<Option<[u32; 12]> as FromResidual<Option<Infallible>>>::from_residual` in `synthyrion_vmm::tablelink::words`
callee `core::slice::get` in `synthyrion_vmm::tablelink::words`
callee `<&mut [u32] as IntoIterator>::into_iter` in `synthyrion_vmm::tablelink::write`
callee `<Link as Default>::default` in `synthyrion_vmm::tablelink::write`
callee `<Link as PartialEq>::eq` in `synthyrion_vmm::tablelink::write`
callee `<core::slice::IterMut<'_, u32> as Iterator>::next` in `synthyrion_vmm::tablelink::write`
callee `core::slice::get_mut` in `synthyrion_vmm::tablelink::write`
loop `synthyrion_vmm::tablelink::write` header bb9
loop `synthyrion_vmm::tables_needed_range_in_empty_tree` header bb9
callee `<&[(u64, u64, u64)] as IntoIterator>::into_iter` in `synthyrion_vmm::to_stored`
loop `synthyrion_vmm::to_stored` header bb4
callee `<Option<&usize> as Try>::branch` in `synthyrion_vmm::treerec::at`
callee `<Option<(usize, usize, usize)> as FromResidual<Option<Infallible>>>::from_residual` in `synthyrion_vmm::treerec::at`
callee `core::slice::get` in `synthyrion_vmm::treerec::at`
callee `core::slice::get_mut` in `synthyrion_vmm::treerec::corrupt`
callee `core::slice::get_mut` in `synthyrion_vmm::treerec::corrupt::{closure#0}`
callee `<&u64 as PartialEq>::eq` in `synthyrion_vmm::treerec::eq`
callee `Formatter::debug_tuple_field1_finish` in `synthyrion_vmm::treerec::fmt`
callee `Formatter::write_str` in `synthyrion_vmm::treerec::fmt`
callee `<&mut [u32] as IntoIterator>::into_iter` in `synthyrion_vmm::treerec::format`
callee `<core::slice::IterMut<'_, u32> as Iterator>::next` in `synthyrion_vmm::treerec::format`
callee `core::slice::get_mut` in `synthyrion_vmm::treerec::format`
loop `synthyrion_vmm::treerec::format` header bb6
callee `<Result<TreeRead, TreeFault> as FromResidual<Result<Infallible, TreeFault>>>::from_residual` in `synthyrion_vmm::treerec::get`
callee `<Result<TreeRead, TreeFault> as Try>::branch` in `synthyrion_vmm::treerec::get`
callee `check_word` in `synthyrion_vmm::treerec::sum`
callee `<Option<&u32> as Try>::branch` in `synthyrion_vmm::treerec::words`
callee `<Option<(usize, usize, usize)> as Try>::branch` in `synthyrion_vmm::treerec::words`
callee `core::slice::get` in `synthyrion_vmm::treerec::words`
callee `Cell::get` in `synthyrion_vmm::treeview::AddressSpace::apply_health`
callee `Cell::set` in `synthyrion_vmm::treeview::AddressSpace::apply_health`
callee `<M as TableMem>::tables_mut` in `synthyrion_vmm::treeview::AddressSpace::attach`
callee `<TableBackend<M> as TableStore>::insert_new` in `synthyrion_vmm::treeview::AddressSpace::attach_new`
callee `<M as TableMem>::tables` in `synthyrion_vmm::treeview::AddressSpace::attach_ro`
callee `<M as TableMem>::tables_mut` in `synthyrion_vmm::treeview::AddressSpace::commit`
loop `synthyrion_vmm::treeview::AddressSpace::commit` header bb10
callee `<M as TableMem>::tables_mut` in `synthyrion_vmm::treeview::AddressSpace::erase_record`
callee `Cell::new` in `synthyrion_vmm::treeview::AddressSpace::from_record`
callee `Cell::set` in `synthyrion_vmm::treeview::AddressSpace::from_record`
callee `Cell::get` in `synthyrion_vmm::treeview::AddressSpace::health`
callee `Cell::get` in `synthyrion_vmm::treeview::AddressSpace::record_now`
callee `<ReadHealth as PartialEq>::eq` in `synthyrion_vmm::treeview::ReadHealth::is_none`
callee `core::slice::get` in `synthyrion_vmm::treeview::at`
callee `Box::new` in `synthyrion_vmm::treeview::clone_box`
callee `<bool as Default>::default` in `synthyrion_vmm::treeview::default`
callee `<u64 as Default>::default` in `synthyrion_vmm::treeview::default`
callee `Formatter::debug_struct_field3_finish` in `synthyrion_vmm::treeview::fmt`
callee `Formatter::write_str` in `synthyrion_vmm::treeview::fmt`
callee `<Result<ArmSwitch, ArmError> as FromResidual<Result<Infallible, ArmError>>>::from_residual` in `synthyrion_vmm::vmid_switch`
callee `<Result<ArmSwitch, ArmError> as Try>::branch` in `synthyrion_vmm::vmid_switch`
callee `core::num::div_ceil` in `synthyrion_vmm::walk`
callee `core::num::is_power_of_two` in `synthyrion_vmm::walk`
callee `Formatter::write_str` in `hypervisor::<Access as Debug>::fmt`
callee `Formatter::write_str` in `hypervisor::<AccessFault as Debug>::fmt`
callee `Formatter::debug_tuple_field1_finish` in `hypervisor::<BuildError as Debug>::fmt`
callee `Formatter::write_str` in `hypervisor::<BuildError as Debug>::fmt`
callee `<&EptEncodeError as PartialEq>::eq` in `hypervisor::<BuildError as PartialEq>::eq`
callee `Formatter::write_str` in `hypervisor::<EptEncodeError as Debug>::fmt`
callee `Formatter::debug_tuple_field1_finish` in `hypervisor::<EptGateError as Debug>::fmt`
callee `Formatter::write_str` in `hypervisor::<EptGateError as Debug>::fmt`
callee `<&EptEncodeError as PartialEq>::eq` in `hypervisor::<EptGateError as PartialEq>::eq`
callee `Formatter::debug_struct_field4_finish` in `hypervisor::<EptViolationFacts as Debug>::fmt`
callee `Formatter::debug_struct_fields_finish` in `hypervisor::<GuestState as Debug>::fmt`
callee `Formatter::debug_struct_fields_finish` in `hypervisor::<HostState as Debug>::fmt`
callee `Formatter::write_str` in `hypervisor::<MapError as Debug>::fmt`
callee `Formatter::debug_struct_field3_finish` in `hypervisor::<Perms as Debug>::fmt`
callee `Formatter::debug_tuple_field1_finish` in `hypervisor::<Protected as Debug>::fmt`
callee `Formatter::debug_struct_field2_finish` in `hypervisor::<Range as Debug>::fmt`
callee `Formatter::debug_struct_fields_finish` in `hypervisor::<SegAccessRights as Debug>::fmt`
callee `Formatter::debug_struct_field4_finish` in `hypervisor::<SegmentState as Debug>::fmt`
callee `Formatter::debug_tuple_field1_finish` in `hypervisor::<VmExitReason as Debug>::fmt`
callee `Formatter::write_str` in `hypervisor::<VmExitReason as Debug>::fmt`
callee `<&u16 as PartialEq>::eq` in `hypervisor::<VmExitReason as PartialEq>::eq`
callee `Formatter::write_str` in `hypervisor::<VmcsError as Debug>::fmt`
callee `Formatter::debug_struct_field5_finish` in `hypervisor::<VmxCaps as Debug>::fmt`
callee `Formatter::write_str` in `hypervisor::<VmxReadiness as Debug>::fmt`
callee `<&Vec<Mapping> as IntoIterator>::into_iter` in `hypervisor::Ept::access`
callee `<core::slice::Iter<'_, Mapping> as Iterator>::next` in `hypervisor::Ept::access`
loop `hypervisor::Ept::access` header bb2
callee `<Vec<Mapping> as Deref>::deref` in `hypervisor::Ept::any_protected_reachable`
callee `<core::slice::Iter<'_, Mapping> as Iterator>::any` in `hypervisor::Ept::any_protected_reachable`
callee `<&Vec<Mapping> as IntoIterator>::into_iter` in `hypervisor::Ept::build_tables`
callee `<A as FrameAllocator>::alloc_frame` in `hypervisor::Ept::build_tables`
callee `<Result<EptTables, BuildError> as FromResidual<Result<Infallible, BuildError>>>::from_residual` in `hypervisor::Ept::build_tables`
callee `<Result<u64, BuildError> as Try>::branch` in `hypervisor::Ept::build_tables`
callee `<core::slice::Iter<'_, Mapping> as Iterator>::next` in `hypervisor::Ept::build_tables`
callee `BTreeMap::get` in `hypervisor::Ept::build_tables`
callee `BTreeMap::get_mut` in `hypervisor::Ept::build_tables`
callee `BTreeMap::insert` in `hypervisor::Ept::build_tables`
callee `BTreeMap::new` in `hypervisor::Ept::build_tables`
callee `Result::map_err BuildError {BuildError::Encode}>` in `hypervisor::Ept::build_tables`
loop `hypervisor::Ept::build_tables` header bb13
callee `<Vec<Protected> as Deref>::deref` in `hypervisor::Ept::hits_protected`
callee `<core::slice::Iter<'_, Protected> as Iterator>::any` in `hypervisor::Ept::hits_protected`
callee `<Vec<Mapping> as Deref>::deref` in `hypervisor::Ept::map_guest_wx`
callee `<core::slice::Iter<'_, Mapping> as Iterator>::any` in `hypervisor::Ept::map_guest_wx`
callee `Vec::push` in `hypervisor::Ept::map_guest_wx`
callee `<Map<core::slice::Iter<'_, Mapping>, {closure@crates\hypervisor\src\lib.rs:321:30: 321:33}> as Iterator>::collect` in `hypervisor::Ept::mappings`
callee `<Vec<Mapping> as Deref>::deref` in `hypervisor::Ept::mappings`
callee `<core::slice::Iter<'_, Mapping> as Iterator>::map` in `hypervisor::Ept::mappings`
callee `Vec::new` in `hypervisor::Ept::new`
callee `<Option<Ept> as FromResidual<Option<Infallible>>>::from_residual` in `hypervisor::Ept::rebuilt_from`
callee `<impl FnMut(u64) -> Option<(u64, u64, u64, Perms)> as FnMut<(u64,)>>::call_mut` in `hypervisor::Ept::rebuilt_from`
loop `hypervisor::Ept::rebuilt_from` header bb2
callee `<&Vec<Mapping> as IntoIterator>::into_iter` in `hypervisor::Ept::translate`
callee `<core::slice::Iter<'_, Mapping> as Iterator>::next` in `hypervisor::Ept::translate`
loop `hypervisor::Ept::translate` header bb2
callee `Vec::len` in `hypervisor::Ept::unmap`
callee `Vec::retain` in `hypervisor::Ept::unmap`
callee `<A as FrameAllocator>::alloc_frame` in `hypervisor::EptTables::descend`
callee `<Result<u64, BuildError> as FromResidual<Result<Infallible, BuildError>>>::from_residual` in `hypervisor::EptTables::descend`
callee `<Result<u64, BuildError> as Try>::branch` in `hypervisor::EptTables::descend`
callee `BTreeMap::get` in `hypervisor::EptTables::descend`
callee `BTreeMap::get_mut` in `hypervisor::EptTables::descend`
callee `BTreeMap::insert` in `hypervisor::EptTables::descend`
callee `BTreeMap::len` in `hypervisor::EptTables::frame_count`
callee `<Option<&[u64; 512]> as Try>::branch` in `hypervisor::EptTables::leaf_perms`
callee `<Option<Perms> as FromResidual<Option<Infallible>>>::from_residual` in `hypervisor::EptTables::leaf_perms`
callee `<[u32; 3] as IntoIterator>::into_iter` in `hypervisor::EptTables::leaf_perms`
callee `<core::array::IntoIter<u32, 3> as Iterator>::next` in `hypervisor::EptTables::leaf_perms`
callee `BTreeMap::get` in `hypervisor::EptTables::leaf_perms`
loop `hypervisor::EptTables::leaf_perms` header bb2
callee `<Option<&[u64; 512]> as Try>::branch` in `hypervisor::EptTables::translate`
callee `<[u32; 3] as IntoIterator>::into_iter` in `hypervisor::EptTables::translate`
callee `<core::array::IntoIter<u32, 3> as Iterator>::next` in `hypervisor::EptTables::translate`
callee `BTreeMap::get` in `hypervisor::EptTables::translate`
loop `hypervisor::EptTables::translate` header bb2
callee `<EventKind as PartialEq>::eq` in `hypervisor::amd_eventinj`
callee `<Result<(), InjectError> as Try>::branch` in `hypervisor::amd_eventinj`
callee `<Result<u64, InjectError> as FromResidual<Result<Infallible, InjectError>>>::from_residual` in `hypervisor::amd_eventinj`
callee `<&InjectError as PartialEq>::eq` in `hypervisor::apply::eq`
callee `<&u64 as PartialEq>::eq` in `hypervisor::apply::eq`
callee `Formatter::debug_tuple_field1_finish` in `hypervisor::apply::fmt`
callee `Formatter::write_str` in `hypervisor::apply::fmt`
callee `<Vec<(u64, u64)> as Deref>::deref` in `hypervisor::apply_msr_passthrough`
callee `<Vec<(u64, u64)> as Index<usize>>::index` in `hypervisor::apply_msr_passthrough`
callee `<Vec<(u64, u64)> as IndexMut<usize>>::index_mut` in `hypervisor::apply_msr_passthrough`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::position` in `hypervisor::apply_msr_passthrough`
callee `<Copied<Take<core::slice::Iter<'_, (u32, u8)>>> as IntoIterator>::into_iter` in `hypervisor::assign_lrs`
callee `<Copied<Take<core::slice::Iter<'_, (u32, u8)>>> as Iterator>::next` in `hypervisor::assign_lrs`
callee `<core::ops::Range<usize> as Iterator>::find` in `hypervisor::assign_lrs`
callee `<usize as Ord>::min` in `hypervisor::assign_lrs`
callee `core::slice::get_mut` in `hypervisor::assign_lrs`
loop `hypervisor::assign_lrs` header bb3
callee `<[usize; 5] as IntoIterator>::into_iter` in `hypervisor::build_vmcb_flat64`
callee `<core::array::IntoIter<usize, 5> as Iterator>::next` in `hypervisor::build_vmcb_flat64`
loop `hypervisor::build_vmcb_flat64` header bb13
callee `<[(u64, u64, u64, u64, &SegmentState); 8] as IntoIterator>::into_iter` in `hypervisor::build_vmcs`
callee `<core::array::IntoIter<(u64, u64, u64, u64, &SegmentState), 8> as Iterator>::next` in `hypervisor::build_vmcs`
callee `Box::new_uninit` in `hypervisor::build_vmcs`
callee `Vec::push` in `hypervisor::build_vmcs`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `hypervisor::build_vmcs`
loop `hypervisor::build_vmcs` header bb18
callee `<[(u64, u64, u64, u64, &SegmentState); 8] as IntoIterator>::into_iter` in `hypervisor::build_vmcs_flat`
callee `<core::array::IntoIter<(u64, u64, u64, u64, &SegmentState), 8> as Iterator>::next` in `hypervisor::build_vmcs_flat`
callee `Box::new_uninit` in `hypervisor::build_vmcs_flat`
callee `Vec::push` in `hypervisor::build_vmcs_flat`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `hypervisor::build_vmcs_flat`
loop `hypervisor::build_vmcs_flat` header bb11
callee `<Result<Vec<(u64, u64)>, VmcsError> as FromResidual<Result<Infallible, VmcsError>>>::from_residual` in `hypervisor::build_vmcs_flat_ept`
callee `<Result<Vec<(u64, u64)>, VmcsError> as Try>::branch` in `hypervisor::build_vmcs_flat_ept`
callee `<Vec<(u64, u64)> as DerefMut>::deref_mut` in `hypervisor::build_vmcs_flat_ept`
callee `<core::slice::IterMut<'_, (u64, u64)> as Iterator>::find` in `hypervisor::build_vmcs_flat_ept`
callee `Vec::push` in `hypervisor::build_vmcs_flat_ept`
callee `<Result<Vec<(u64, u64)>, VmcsError> as FromResidual<Result<Infallible, VmcsError>>>::from_residual` in `hypervisor::build_vmcs_real_ept`
callee `<Result<Vec<(u64, u64)>, VmcsError> as Try>::branch` in `hypervisor::build_vmcs_real_ept`
callee `<Vec<(u64, u64)> as DerefMut>::deref_mut` in `hypervisor::build_vmcs_real_ept`
callee `<core::slice::IterMut<'_, (u64, u64)> as IntoIterator>::into_iter` in `hypervisor::build_vmcs_real_ept`
callee `<core::slice::IterMut<'_, (u64, u64)> as Iterator>::next` in `hypervisor::build_vmcs_real_ept`
callee `Vec::push` in `hypervisor::build_vmcs_real_ept`
loop `hypervisor::build_vmcs_real_ept` header bb24
callee `<EventKind as PartialEq>::ne` in `hypervisor::check_common`
callee `<Option<(u64, Stage2Mem)> as FromResidual<Option<Infallible>>>::from_residual` in `hypervisor::contiguous`
callee `<Option<(u64, Stage2Mem)> as Try>::branch` in `hypervisor::contiguous`
callee `<Stage2Mem as PartialEq>::ne` in `hypervisor::contiguous`
callee `<dyn Fn(u64) -> Option<(u64, Stage2Mem)> as Fn<(u64,)>>::call` in `hypervisor::contiguous`
loop `hypervisor::contiguous` header bb12
callee `move _26` in `hypervisor::cpuid_policy`
callee `move _28` in `hypervisor::cpuid_policy`
callee `move _30` in `hypervisor::cpuid_policy`
callee `move _5` in `hypervisor::cpuid_policy`
callee `move _7` in `hypervisor::cpuid_policy`
callee `move _9` in `hypervisor::cpuid_policy`
callee `<ExitEffects as Default>::default` in `hypervisor::decide_exit`
callee `<Map<core::slice::Iter<'_, El1Reg>, {closure@crates\hypervisor\src\el1state.rs:92:43: 92:46}> as Iterator>::collect` in `hypervisor::el1state::El1State::fresh`
callee `<core::slice::Iter<'_, El1Reg> as Iterator>::map` in `hypervisor::el1state::El1State::fresh`
callee `<El1Reg as PartialEq>::eq` in `hypervisor::el1state::El1State::fresh::{closure#0}`
callee `<Vec<(El1Reg, u64)> as Deref>::deref` in `hypervisor::el1state::El1State::get`
callee `<core::slice::Iter<'_, (El1Reg, u64)> as Iterator>::find` in `hypervisor::el1state::El1State::get`
callee `<El1Reg as PartialEq>::eq` in `hypervisor::el1state::El1State::get::{closure#0}`
callee `<Vec<(El1Reg, u64)> as Clone>::clone` in `hypervisor::el1state::clone`
callee `<Vec<(El1Reg, u64)> as PartialEq>::eq` in `hypervisor::el1state::eq`
callee `Formatter::debug_struct_field1_finish` in `hypervisor::el1state::fmt`
callee `Formatter::write_str` in `hypervisor::el1state::fmt`
callee `<Vec<(u64, u64)> as DerefMut>::deref_mut` in `hypervisor::enable_preemption_timer`
callee `<core::slice::IterMut<'_, (u64, u64)> as Iterator>::find` in `hypervisor::enable_preemption_timer`
callee `Vec::push` in `hypervisor::enable_preemption_timer`
callee `<core::slice::IterMut<'_, (u64, u64)> as IntoIterator>::into_iter` in `hypervisor::enable_rdtsc_exiting`
callee `<core::slice::IterMut<'_, (u64, u64)> as Iterator>::next` in `hypervisor::enable_rdtsc_exiting`
loop `hypervisor::enable_rdtsc_exiting` header bb3
callee `<core::slice::IterMut<'_, (u64, u64)> as IntoIterator>::into_iter` in `hypervisor::enable_runloop_controls`
callee `<core::slice::IterMut<'_, (u64, u64)> as Iterator>::next` in `hypervisor::enable_runloop_controls`
loop `hypervisor::enable_runloop_controls` header bb3
callee `Result::map_err EptGateError {EptGateError::Encode}>` in `hypervisor::ept_leaf_entry_gated`
callee `<Map<core::slice::Iter<'_, u64>, {closure@crates\hypervisor\src\inject.rs:305:29: 305:32}> as Iterator>::sum` in `hypervisor::inject::IrqQueue::count`
callee `<core::slice::Iter<'_, u64> as Iterator>::map` in `hypervisor::inject::IrqQueue::count`
callee `<Rev<core::ops::Range<usize>> as IntoIterator>::into_iter` in `hypervisor::inject::IrqQueue::highest`
callee `<Rev<core::ops::Range<usize>> as Iterator>::next` in `hypervisor::inject::IrqQueue::highest`
callee `<core::ops::Range<usize> as Iterator>::rev` in `hypervisor::inject::IrqQueue::highest`
callee `core::num::leading_zeros` in `hypervisor::inject::IrqQueue::highest`
loop `hypervisor::inject::IrqQueue::highest` header bb3
callee `<Option<(Event, Deliver)> as FromResidual<Option<Infallible>>>::from_residual` in `hypervisor::inject::IrqQueue::next`
callee `<Option<u8> as Try>::branch` in `hypervisor::inject::IrqQueue::next`
callee `<[u64; 4] as Default>::default` in `hypervisor::inject::default`
callee `<bool as Default>::default` in `hypervisor::inject::default`
callee `<&Event as PartialEq>::eq` in `hypervisor::inject::eq`
callee `<&u8 as PartialEq>::eq` in `hypervisor::inject::eq`
callee `<EventKind as PartialEq>::eq` in `hypervisor::inject::eq`
callee `<Option<u32> as PartialEq>::eq` in `hypervisor::inject::eq`
callee `<[u64; 4] as PartialEq>::eq` in `hypervisor::inject::eq`
callee `Formatter::debug_struct_field1_finish` in `hypervisor::inject::fmt`
callee `Formatter::debug_struct_field3_finish` in `hypervisor::inject::fmt`
callee `Formatter::debug_struct_field4_finish` in `hypervisor::inject::fmt`
callee `Formatter::debug_struct_field5_finish` in `hypervisor::inject::fmt`
callee `Formatter::debug_tuple_field1_finish` in `hypervisor::inject::fmt`
callee `Formatter::write_str` in `hypervisor::inject::fmt`
callee `<EventKind as PartialEq>::eq` in `hypervisor::intel_entry_info`
callee `<Result<(), InjectError> as Try>::branch` in `hypervisor::intel_entry_info`
callee `<Result<IntelInjection, InjectError> as FromResidual<Result<Infallible, InjectError>>>::from_residual` in `hypervisor::intel_entry_info`
callee `<Inject as PartialEq>::eq` in `hypervisor::intel_prepare_entry`
callee `<Result<(), ApplyError> as FromResidual<Result<Infallible, ApplyError>>>::from_residual` in `hypervisor::intel_prepare_entry`
callee `<Result<(), ApplyError> as Try>::branch` in `hypervisor::intel_prepare_entry`
callee `<Result<u64, ApplyError> as Try>::branch` in `hypervisor::intel_prepare_entry`
callee `<Result<(bool, bool, bool), ApplyError> as FromResidual<Result<Infallible, ApplyError>>>::from_residual` in `hypervisor::intel_read_intr_state`
callee `<Result<u64, ApplyError> as Try>::branch` in `hypervisor::intel_read_intr_state`
callee `<EventKind as PartialEq>::eq` in `hypervisor::intel_write_event`
callee `<Result<(), ApplyError> as FromResidual<Result<Infallible, ApplyError>>>::from_residual` in `hypervisor::intel_write_event`
callee `<Result<(), ApplyError> as FromResidual<Result<Infallible, InjectError>>>::from_residual` in `hypervisor::intel_write_event`
callee `<Result<(), ApplyError> as Try>::branch` in `hypervisor::intel_write_event`
callee `<Result<IntelInjection, InjectError> as Try>::branch` in `hypervisor::intel_write_event`
callee `Formatter::write_str` in `hypervisor::maint::fmt`
callee `<Copied<core::slice::Iter<'_, u64>> as Iterator>::filter` in `hypervisor::missing_required_fields`
callee `<Filter<Copied<core::slice::Iter<'_, u64>>, {closure@crates\hypervisor\src\lib.rs:1874:17: 1874:20}> as Iterator>::collect` in `hypervisor::missing_required_fields`
callee `<core::slice::Iter<'_, u64> as Iterator>::copied` in `hypervisor::missing_required_fields`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::any` in `hypervisor::missing_required_fields::{closure#0}`
callee `<&u64 as PartialEq>::eq` in `hypervisor::missing_required_fields::{closure#0}::{closure#0}`
callee `<Result<(), MsrBitmapError> as FromResidual<Result<Infallible, MsrBitmapError>>>::from_residual` in `hypervisor::msr_bitmap_allow`
callee `<Result<(usize, usize, u8), MsrBitmapError> as Try>::branch` in `hypervisor::msr_bitmap_allow`
callee `<Chain<core::slice::Iter<'_, u32>, core::slice::Iter<'_, u32>> as IntoIterator>::into_iter` in `hypervisor::msr_bitmap_for_guest`
callee `<Chain<core::slice::Iter<'_, u32>, core::slice::Iter<'_, u32>> as Iterator>::next` in `hypervisor::msr_bitmap_for_guest`
callee `<core::slice::Iter<'_, u32> as Iterator>::chain` in `hypervisor::msr_bitmap_for_guest`
loop `hypervisor::msr_bitmap_for_guest` header bb6
callee `<V as VmcsIo>::read` in `hypervisor::rd`
callee `<R as El1Regs>::write` in `hypervisor::restore`
callee `<[El1Reg; 24] as IntoIterator>::into_iter` in `hypervisor::restore`
callee `<core::array::IntoIter<El1Reg, 24> as Iterator>::next` in `hypervisor::restore`
loop `hypervisor::restore` header bb2
callee `<Option<u64> as Default>::default` in `hypervisor::runloop::default`
callee `<Option<u8> as Default>::default` in `hypervisor::runloop::default`
callee `<u64 as Default>::default` in `hypervisor::runloop::default`
callee `<&bool as PartialEq>::eq` in `hypervisor::runloop::eq`
callee `<&u32 as PartialEq>::eq` in `hypervisor::runloop::eq`
callee `<Option<u8> as PartialEq>::eq` in `hypervisor::runloop::eq`
callee `Formatter::debug_struct_field1_finish` in `hypervisor::runloop::fmt`
callee `Formatter::debug_struct_field3_finish` in `hypervisor::runloop::fmt`
callee `Formatter::debug_struct_field5_finish` in `hypervisor::runloop::fmt`
callee `Formatter::debug_struct_fields_finish` in `hypervisor::runloop::fmt`
callee `Formatter::debug_tuple_field1_finish` in `hypervisor::runloop::fmt`
callee `Formatter::write_str` in `hypervisor::runloop::fmt`
callee `<&[u8] as PartialEq>::ne` in `hypervisor::runloop_verdict`
callee `<Result<u64, Stage2EncodeError> as FromResidual<Result<Infallible, Stage2EncodeError>>>::from_residual` in `hypervisor::s2_block_entry`
callee `<Result<u64, Stage2EncodeError> as Try>::branch` in `hypervisor::s2_block_entry`
callee `<R as El1Regs>::read` in `hypervisor::save`
callee `<Vec<(El1Reg, u64)> as DerefMut>::deref_mut` in `hypervisor::save`
callee `<[El1Reg; 24] as IntoIterator>::into_iter` in `hypervisor::save`
callee `<core::array::IntoIter<El1Reg, 24> as Iterator>::next` in `hypervisor::save`
callee `<core::slice::IterMut<'_, (El1Reg, u64)> as Iterator>::find` in `hypervisor::save`
loop `hypervisor::save` header bb2
callee `<El1Reg as PartialEq>::eq` in `hypervisor::save::{closure#0}`
callee `<Result<(), InjectError> as FromResidual<Result<Infallible, InjectError>>>::from_residual` in `hypervisor::set_event`
callee `<Result<u64, InjectError> as Try>::branch` in `hypervisor::set_event`
callee `<A as FrameAllocator>::alloc_frame` in `hypervisor::stage2::Ept::build_stage2`
callee `<Result<(u64, Stage2Mem), Stage2BuildError> as Try>::branch` in `hypervisor::stage2::Ept::build_stage2`
callee `<Result<Stage2Tables, Stage2BuildError> as FromResidual<Result<Infallible, Stage2BuildError>>>::from_residual` in `hypervisor::stage2::Ept::build_stage2`
callee `<Result<u64, Stage2BuildError> as Try>::branch` in `hypervisor::stage2::Ept::build_stage2`
callee `<Vec<(u64, u64, u64, Perms)> as DerefMut>::deref_mut` in `hypervisor::stage2::Ept::build_stage2`
callee `<Vec<(u64, u64, u64, Perms)> as IntoIterator>::into_iter` in `hypervisor::stage2::Ept::build_stage2`
callee `<alloc::vec::IntoIter<(u64, u64, u64, Perms)> as Iterator>::next` in `hypervisor::stage2::Ept::build_stage2`
callee `<dyn Fn(u64) -> Option<(u64, Stage2Mem)> as Fn<(u64,)>>::call` in `hypervisor::stage2::Ept::build_stage2`
callee `BTreeMap::get_mut` in `hypervisor::stage2::Ept::build_stage2`
callee `BTreeMap::insert` in `hypervisor::stage2::Ept::build_stage2`
callee `BTreeMap::new` in `hypervisor::stage2::Ept::build_stage2`
callee `Result::map_err Stage2BuildError {Stage2BuildError::Encode}>` in `hypervisor::stage2::Ept::build_stage2`
callee `Vec::new` in `hypervisor::stage2::Ept::build_stage2`
callee `Vec::push` in `hypervisor::stage2::Ept::build_stage2`
callee `alloc::slice::sort_by_key` in `hypervisor::stage2::Ept::build_stage2`
callee `core::slice::last_mut` in `hypervisor::stage2::Ept::build_stage2`
loop `hypervisor::stage2::Ept::build_stage2` header bb14
loop `hypervisor::stage2::Ept::build_stage2` header bb33
callee `core::slice::get_mut` in `hypervisor::stage2::Ept::build_stage2::{closure#1}`
callee `core::slice::get_mut` in `hypervisor::stage2::Ept::build_stage2::{closure#2}`
callee `<&u64 as BitAnd<u64>>::bitand` in `hypervisor::stage2::Stage2Tables::block_count`
callee `<alloc::collections::btree_map::Iter<'_, u64, [u64; 512]> as IntoIterator>::into_iter` in `hypervisor::stage2::Stage2Tables::block_count`
callee `<alloc::collections::btree_map::Iter<'_, u64, [u64; 512]> as Iterator>::next` in `hypervisor::stage2::Stage2Tables::block_count`
callee `BTreeMap::iter` in `hypervisor::stage2::Stage2Tables::block_count`
loop `hypervisor::stage2::Stage2Tables::block_count` header bb3
callee `<A as FrameAllocator>::alloc_frame` in `hypervisor::stage2::Stage2Tables::descend`
callee `<Result<u64, Stage2BuildError> as FromResidual<Result<Infallible, Stage2BuildError>>>::from_residual` in `hypervisor::stage2::Stage2Tables::descend`
callee `<Result<u64, Stage2BuildError> as Try>::branch` in `hypervisor::stage2::Stage2Tables::descend`
callee `BTreeMap::get` in `hypervisor::stage2::Stage2Tables::descend`
callee `BTreeMap::get_mut` in `hypervisor::stage2::Stage2Tables::descend`
callee `BTreeMap::insert` in `hypervisor::stage2::Stage2Tables::descend`
callee `core::slice::get` in `hypervisor::stage2::Stage2Tables::descend::{closure#0}`
callee `core::slice::get_mut` in `hypervisor::stage2::Stage2Tables::descend::{closure#1}`
callee `BTreeMap::len` in `hypervisor::stage2::Stage2Tables::frame_count`
callee `<Option<(u64, u64)> as Try>::branch` in `hypervisor::stage2::Stage2Tables::leaf_mem`
callee `<Option<Stage2Mem> as FromResidual<Option<Infallible>>>::from_residual` in `hypervisor::stage2::Stage2Tables::leaf_mem`
callee `<Option<(u64, u64)> as Try>::branch` in `hypervisor::stage2::Stage2Tables::leaf_perms`
callee `<Option<Perms> as FromResidual<Option<Infallible>>>::from_residual` in `hypervisor::stage2::Stage2Tables::leaf_perms`
callee `<Option<(u64, u64)> as Try>::branch` in `hypervisor::stage2::Stage2Tables::translate`
callee `<Result<u64, AccessFault> as FromResidual<Result<Infallible, AccessFault>>>::from_residual` in `hypervisor::stage2::Stage2Tables::walk`
callee `<Result<u64, AccessFault> as Try>::branch` in `hypervisor::stage2::Stage2Tables::walk`
callee `<Option<&[u64; 512]> as Try>::branch` in `hypervisor::stage2::Stage2Tables::walk_desc`
callee `BTreeMap::get` in `hypervisor::stage2::Stage2Tables::walk_desc`
callee `core::slice::get` in `hypervisor::stage2::Stage2Tables::walk_desc`
callee `<&BuildError as PartialEq>::eq` in `hypervisor::stage2::eq`
callee `<&Stage2EncodeError as PartialEq>::eq` in `hypervisor::stage2::eq`
callee `<&u8 as PartialEq>::eq` in `hypervisor::stage2::eq`
callee `<Stage2FaultKind as PartialEq>::eq` in `hypervisor::stage2::eq`
callee `Formatter::debug_struct_fields_finish` in `hypervisor::stage2::fmt`
callee `Formatter::debug_tuple_field1_finish` in `hypervisor::stage2::fmt`
callee `Formatter::write_str` in `hypervisor::stage2::fmt`
callee `<&Vec<Mapping> as IntoIterator>::into_iter` in `hypervisor::svm::Ept::build_npt`
callee `<A as FrameAllocator>::alloc_frame` in `hypervisor::svm::Ept::build_npt`
callee `<Result<NptTables, NptBuildError> as FromResidual<Result<Infallible, NptBuildError>>>::from_residual` in `hypervisor::svm::Ept::build_npt`
callee `<Result<u64, NptBuildError> as Try>::branch` in `hypervisor::svm::Ept::build_npt`
callee `<core::slice::Iter<'_, Mapping> as Iterator>::next` in `hypervisor::svm::Ept::build_npt`
callee `BTreeMap::get_mut` in `hypervisor::svm::Ept::build_npt`
callee `BTreeMap::insert` in `hypervisor::svm::Ept::build_npt`
callee `BTreeMap::new` in `hypervisor::svm::Ept::build_npt`
callee `Result::map_err NptBuildError {NptBuildError::Encode}>` in `hypervisor::svm::Ept::build_npt`
loop `hypervisor::svm::Ept::build_npt` header bb10
callee `<A as FrameAllocator>::alloc_frame` in `hypervisor::svm::NptTables::descend`
callee `<Result<u64, NptBuildError> as FromResidual<Result<Infallible, NptBuildError>>>::from_residual` in `hypervisor::svm::NptTables::descend`
callee `<Result<u64, NptBuildError> as Try>::branch` in `hypervisor::svm::NptTables::descend`
callee `BTreeMap::get` in `hypervisor::svm::NptTables::descend`
callee `BTreeMap::get_mut` in `hypervisor::svm::NptTables::descend`
callee `BTreeMap::insert` in `hypervisor::svm::NptTables::descend`
callee `BTreeMap::len` in `hypervisor::svm::NptTables::frame_count`
callee `<Option<&[u64; 512]> as Try>::branch` in `hypervisor::svm::NptTables::leaf_perms`
callee `<Option<(u64, usize)> as Try>::branch` in `hypervisor::svm::NptTables::leaf_perms`
callee `<Option<Perms> as FromResidual<Option<Infallible>>>::from_residual` in `hypervisor::svm::NptTables::leaf_perms`
callee `BTreeMap::get` in `hypervisor::svm::NptTables::leaf_perms`
callee `<Option<&[u64; 512]> as Try>::branch` in `hypervisor::svm::NptTables::leaf_slot`
callee `<Option<(u64, usize)> as FromResidual<Option<Infallible>>>::from_residual` in `hypervisor::svm::NptTables::leaf_slot`
callee `<[u32; 3] as IntoIterator>::into_iter` in `hypervisor::svm::NptTables::leaf_slot`
callee `<core::array::IntoIter<u32, 3> as Iterator>::next` in `hypervisor::svm::NptTables::leaf_slot`
callee `BTreeMap::get` in `hypervisor::svm::NptTables::leaf_slot`
loop `hypervisor::svm::NptTables::leaf_slot` header bb2
callee `<Option<&[u64; 512]> as Try>::branch` in `hypervisor::svm::NptTables::translate`
callee `<Option<(u64, usize)> as Try>::branch` in `hypervisor::svm::NptTables::translate`
callee `BTreeMap::get` in `hypervisor::svm::NptTables::translate`
callee `<Result<u64, AccessFault> as FromResidual<Result<Infallible, AccessFault>>>::from_residual` in `hypervisor::svm::NptTables::walk`
callee `<Result<u64, AccessFault> as Try>::branch` in `hypervisor::svm::NptTables::walk`
callee `<&[u8] as TryInto<[u8; 2]>>::try_into` in `hypervisor::svm::Vmcb::get16`
callee `<[u8; 4096] as Index<core::ops::Range<usize>>>::index` in `hypervisor::svm::Vmcb::get16`
callee `core::num::from_le_bytes` in `hypervisor::svm::Vmcb::get16`
callee `<&[u8] as TryInto<[u8; 4]>>::try_into` in `hypervisor::svm::Vmcb::get32`
callee `<[u8; 4096] as Index<core::ops::Range<usize>>>::index` in `hypervisor::svm::Vmcb::get32`
callee `core::num::from_le_bytes` in `hypervisor::svm::Vmcb::get32`
callee `<&[u8] as TryInto<[u8; 8]>>::try_into` in `hypervisor::svm::Vmcb::get64`
callee `<[u8; 4096] as Index<core::ops::Range<usize>>>::index` in `hypervisor::svm::Vmcb::get64`
callee `core::num::from_le_bytes` in `hypervisor::svm::Vmcb::get64`
callee `<[u8; 4096] as IndexMut<core::ops::Range<usize>>>::index_mut` in `hypervisor::svm::Vmcb::put16`
callee `core::slice::copy_from_slice` in `hypervisor::svm::Vmcb::put16`
callee `<[u8; 4096] as IndexMut<core::ops::Range<usize>>>::index_mut` in `hypervisor::svm::Vmcb::put32`
callee `core::slice::copy_from_slice` in `hypervisor::svm::Vmcb::put32`
callee `<[u8; 4096] as IndexMut<core::ops::Range<usize>>>::index_mut` in `hypervisor::svm::Vmcb::put64`
callee `core::slice::copy_from_slice` in `hypervisor::svm::Vmcb::put64`
callee `<&BuildError as PartialEq>::eq` in `hypervisor::svm::eq`
callee `<&NptEncodeError as PartialEq>::eq` in `hypervisor::svm::eq`
callee `<&u64 as PartialEq>::eq` in `hypervisor::svm::eq`
callee `Formatter::debug_struct_field2_finish` in `hypervisor::svm::fmt`
callee `Formatter::debug_struct_field3_finish` in `hypervisor::svm::fmt`
callee `Formatter::debug_struct_fields_finish` in `hypervisor::svm::fmt`
callee `Formatter::debug_tuple_field1_finish` in `hypervisor::svm::fmt`
callee `Formatter::write_str` in `hypervisor::svm::fmt`
callee `<EventKind as PartialEq>::eq` in `hypervisor::svm_prepare_entry`
callee `<Result<(), ApplyError> as FromResidual<Result<Infallible, InjectError>>>::from_residual` in `hypervisor::svm_prepare_entry`
callee `<Result<u64, InjectError> as Try>::branch` in `hypervisor::svm_prepare_entry`
callee `<Vec<(u64, u64)> as DerefMut>::deref_mut` in `hypervisor::upsert`
callee `<core::slice::IterMut<'_, (u64, u64)> as Iterator>::find` in `hypervisor::upsert`
callee `Vec::push` in `hypervisor::upsert`
callee `core::num::from_le_bytes` in `hypervisor::vendor_word`
callee `<Take<core::slice::Iter<'_, (u32, u8)>> as Iterator>::copied` in `hypervisor::vgic::VirqBatch::iter`
callee `<core::slice::Iter<'_, (u32, u8)> as Iterator>::take` in `hypervisor::vgic::VirqBatch::iter`
callee `core::slice::get_mut` in `hypervisor::vgic::VirqBatch::push`
callee `<[(u32, u8); 4] as PartialEq>::eq` in `hypervisor::vgic::eq`
callee `Formatter::debug_struct_field2_finish` in `hypervisor::vgic::fmt`
callee `Formatter::debug_struct_field4_finish` in `hypervisor::vgic::fmt`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::find` in `hypervisor::vmcs_value`
callee `<u32 as Ord>::min` in `hypervisor::vmrun_checks`
callee `Formatter::write_str` in `hypervisor::vtimer::fmt`
callee `<V as VmcsIo>::write` in `hypervisor::wr`
callee `Formatter::debug_struct_field1_finish` in `microkernel::<Delivery as Debug>::fmt`
callee `Formatter::debug_struct_field2_finish` in `microkernel::<Delivery as Debug>::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::<Delivery as Debug>::fmt`
callee `<&Rich as PartialEq>::eq` in `microkernel::<Delivery as PartialEq>::eq`
callee `<&u64 as PartialEq>::eq` in `microkernel::<Delivery as PartialEq>::eq`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::<Dispatch as Debug>::fmt`
callee `Formatter::write_str` in `microkernel::<Dispatch as Debug>::fmt`
callee `<&u64 as PartialEq>::eq` in `microkernel::<Dispatch as PartialEq>::eq`
callee `Formatter::write_str` in `microkernel::<MemError as Debug>::fmt`
callee `Formatter::debug_struct_field2_finish` in `microkernel::<RtParams as Debug>::fmt`
callee `<alloc::vec::Vec<u8> as Clone>::clone` in `microkernel::<VaultEvidence as Clone>::clone`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::<VaultEvidence as Debug>::fmt`
callee `<&[u8; 32] as PartialEq>::eq` in `microkernel::<VaultEvidence as PartialEq>::eq`
callee `<&alloc::vec::Vec<u8> as PartialEq>::eq` in `microkernel::<VaultEvidence as PartialEq>::eq`
callee `Formatter::write_str` in `microkernel::<WeightError as Debug>::fmt`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as IntoIterator>::into_iter` in `microkernel::Kernel::address_space_orphans`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as Iterator>::next` in `microkernel::Kernel::address_space_orphans`
callee `is_some_and (2 closure position(s) seen but 1 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::Kernel::address_space_orphans`
loop `microkernel::Kernel::address_space_orphans` header bb6
callee `synthyrion_vmm::treeview::apply_health` in `microkernel::Kernel::apply_read_health::{closure#0}`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as Iterator>::any` in `microkernel::Kernel::attach_table_window_factory`
callee `<FilterMap<alloc::vec::IntoIter<Delivery>, {closure@crates\microkernel\src\lib.rs:2396:25: 2396:28}> as Iterator>::collect` in `microkernel::Kernel::bounce_inbox_of`
callee `<alloc::vec::IntoIter<(u64, u64)> as Iterator>::next` in `microkernel::Kernel::bounce_inbox_of`
callee `<alloc::vec::IntoIter<Delivery> as Iterator>::filter_map` in `microkernel::Kernel::bounce_inbox_of`
callee `<alloc::vec::Vec<(u64, u64)> as IntoIterator>::into_iter` in `microkernel::Kernel::bounce_inbox_of`
callee `<alloc::vec::Vec<Delivery> as IntoIterator>::into_iter` in `microkernel::Kernel::bounce_inbox_of`
loop `microkernel::Kernel::bounce_inbox_of` header bb8
callee `<KernelB as Default>::default` in `microkernel::Kernel::build`
callee `<KernelI as Default>::default` in `microkernel::Kernel::build`
callee `<KernelS as Default>::default` in `microkernel::Kernel::build`
callee `<KernelTp as Default>::default` in `microkernel::Kernel::build`
callee `<LimitStats as Default>::default` in `microkernel::Kernel::build`
callee `<selfheal::KernelH as Default>::default` in `microkernel::Kernel::build`
callee `<vcpu::KernelH as Default>::default` in `microkernel::Kernel::build`
callee `Box::new` in `microkernel::Kernel::build`
callee `FrameAllocator::total` in `microkernel::Kernel::build`
callee `VecDeque::with_capacity` in `microkernel::Kernel::build`
callee `<&[(u64, u64)] as IntoIterator>::into_iter` in `microkernel::Kernel::check_memory_reserve`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::any` in `microkernel::Kernel::check_memory_reserve`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::next` in `microkernel::Kernel::check_memory_reserve`
loop `microkernel::Kernel::check_memory_reserve` header bb12
callee `<Enumerate<core::slice::Iter<'_, (u64, u64)>> as IntoIterator>::into_iter` in `microkernel::Kernel::dispatch_unmap`
callee `<Enumerate<core::slice::Iter<'_, (u64, u64)>> as Iterator>::next` in `microkernel::Kernel::dispatch_unmap`
callee `<alloc::vec::IntoIter<(u64, u64)> as Iterator>::next` in `microkernel::Kernel::dispatch_unmap`
callee `<alloc::vec::Vec<(u64, u64)> as IntoIterator>::into_iter` in `microkernel::Kernel::dispatch_unmap`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::enumerate` in `microkernel::Kernel::dispatch_unmap`
loop `microkernel::Kernel::dispatch_unmap` header bb46
loop `microkernel::Kernel::dispatch_unmap` header bb52
callee `AddressSpace::range_occupancy_healing` in `microkernel::Kernel::dispatch_unmap::{closure#0}`
callee `AddressSpace::unmap_large` in `microkernel::Kernel::dispatch_unmap::{closure#1}`
callee `AddressSpace::translate` in `microkernel::Kernel::dispatch_unmap::{closure#2}`
callee `AddressSpace::unmap` in `microkernel::Kernel::dispatch_unmap::{closure#2}`
callee `alloc::vec::Vec::push` in `microkernel::Kernel::dispatch_unmap::{closure#2}`
loop `microkernel::Kernel::dispatch_unmap::{closure#2}` header bb3
callee `VecDeque::push_back` in `microkernel::Kernel::dispatch_yield`
callee `VecDeque::retain` in `microkernel::Kernel::dispatch_yield`
callee `<Chain<core::iter::Map<Chain<core::slice::Iter<'_, u64>, core::slice::Iter<'_, u64>>, {closure@crates\microkernel\src\lib.rs:1321:85: 1321:89}>, Once<u64>> as Iterator>::filter` in `microkernel::Kernel::dispose_inner`
callee `<Chain<core::slice::Iter<'_, u64>, core::slice::Iter<'_, u64>> as Iterator>::map` in `microkernel::Kernel::dispose_inner`
callee `<Filter<Chain<core::iter::Map<Chain<core::slice::Iter<'_, u64>, core::slice::Iter<'_, u64>>, {closure@crates\microkernel\src\lib.rs:1321:85: 1321:89}>, Once<u64>>, {closure@crates\microkernel\src\lib.rs:1321:212: 1321:216}> as Iterator>::collect` in `microkernel::Kernel::dispose_inner`
callee `<KView<'_> as synthyrion_vmm::TableMem>::unprovide` in `microkernel::Kernel::dispose_inner`
callee `<Option<SpaceBirth> as PartialEq>::ne` in `microkernel::Kernel::dispose_inner`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::Kernel::dispose_inner`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::Kernel::dispose_inner`
callee `<core::iter::Map<Chain<core::slice::Iter<'_, u64>, core::slice::Iter<'_, u64>>, {closure@crates\microkernel\src\lib.rs:1321:85: 1321:89}> as Iterator>::chain` in `microkernel::Kernel::dispose_inner`
callee `AddressSpace::drain_table_frames_with_pairs` in `microkernel::Kernel::dispose_inner`
callee `AddressSpace::leaf_frames` in `microkernel::Kernel::dispose_inner`
callee `AddressSpace::table_frames` in `microkernel::Kernel::dispose_inner`
callee `AddressSpace::window_table_frames` in `microkernel::Kernel::dispose_inner`
callee `synthyrion_vmm::treeview::attach` in `microkernel::Kernel::dispose_inner`
callee `synthyrion_vmm::treeview::erase_record` in `microkernel::Kernel::dispose_inner`
callee `once` in `microkernel::Kernel::dispose_inner`
loop `microkernel::Kernel::dispose_inner` header bb35
loop `microkernel::Kernel::dispose_inner` header bb42
loop `microkernel::Kernel::dispose_inner` header bb59
loop `microkernel::Kernel::dispose_inner` header bb67
loop `microkernel::Kernel::dispose_inner` header bb74
loop `microkernel::Kernel::dispose_inner` header bb80
callee `Cap::new` in `microkernel::Kernel::endpoint_cap`
callee `map (closure Kernel::inbox_detail::{closure#0} is itself unbounded)` in `microkernel::Kernel::inbox_detail`
callee `<alloc::vec::Vec<Delivery> as Deref>::deref` in `microkernel::Kernel::inbox_detail::{closure#0}`
callee `<core::iter::Map<core::slice::Iter<'_, Delivery>, {closure@crates\microkernel\src\lib.rs:2290:61: 2290:64}> as Iterator>::collect` in `microkernel::Kernel::inbox_detail::{closure#0}`
callee `<core::slice::Iter<'_, Delivery> as Iterator>::map` in `microkernel::Kernel::inbox_detail::{closure#0}`
callee `map (closure Kernel::inbox_snapshot::{closure#0} is itself unbounded)` in `microkernel::Kernel::inbox_snapshot`
callee `<alloc::vec::Vec<Delivery> as Deref>::deref` in `microkernel::Kernel::inbox_snapshot::{closure#0}`
callee `<core::iter::Map<core::slice::Iter<'_, Delivery>, {closure@crates\microkernel\src\lib.rs:2283:85: 2283:88}> as Iterator>::collect` in `microkernel::Kernel::inbox_snapshot::{closure#0}`
callee `<core::slice::Iter<'_, Delivery> as Iterator>::map` in `microkernel::Kernel::inbox_snapshot::{closure#0}`
callee `Option::insert` in `microkernel::Kernel::init_in_with_arena_reserve`
callee `Option::insert` in `microkernel::Kernel::init_in_with_memory_or_model_reserve`
callee `<Filter<FilterMap<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:243:50: 243:53}>, {closure@crates\microkernel\src\lib.rs:924:43: 924:51}> as Iterator>::map` in `microkernel::Kernel::legacy_born_spaces`
callee `<FilterMap<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:243:50: 243:53}> as Iterator>::filter` in `microkernel::Kernel::legacy_born_spaces`
callee `<core::iter::Map<Filter<FilterMap<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:243:50: 243:53}>, {closure@crates\microkernel\src\lib.rs:924:43: 924:51}>, {closure@crates\microkernel\src\lib.rs:924:116: 924:124}> as Iterator>::collect` in `microkernel::Kernel::legacy_born_spaces`
callee `<SpaceBirth as PartialEq>::eq` in `microkernel::Kernel::legacy_born_spaces::{closure#0}`
callee `<u64 as Ord>::clamp` in `microkernel::Kernel::set_weight::{closure#0}`
callee `<SpaceHeapCensus as Default>::default` in `microkernel::Kernel::space_heap_census`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as IntoIterator>::into_iter` in `microkernel::Kernel::space_heap_census`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as Iterator>::next` in `microkernel::Kernel::space_heap_census`
callee `AddressSpace::heap_table_pages` in `microkernel::Kernel::space_heap_census`
callee `AddressSpace::unsupplied_table_count` in `microkernel::Kernel::space_heap_census`
callee `AddressSpace::window_table_frames` in `microkernel::Kernel::space_heap_census`
callee `ReadHealth::merge` in `microkernel::Kernel::space_heap_census`
callee `RoView::health` in `microkernel::Kernel::space_heap_census`
callee `core::mem::size_of` in `microkernel::Kernel::space_heap_census`
loop `microkernel::Kernel::space_heap_census` header bb4
callee `<Result<u64, synthyrion_abi::SysResult> as FromResidual<Result<Infallible, synthyrion_abi::SysResult>>>::from_residual` in `microkernel::Kernel::spawn_authorized_rt`
callee `<Result<u64, synthyrion_abi::SysResult> as Try>::branch` in `microkernel::Kernel::spawn_authorized_rt`
callee `<u64 as Ord>::clamp` in `microkernel::Kernel::spawn_task_in_frame`
callee `<alloc::collections::btree_map::Iter<'_, u64, tasktable::Task> as Iterator>::map` in `microkernel::Kernel::stride_snapshot`
callee `<core::iter::Map<alloc::collections::btree_map::Iter<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\lib.rs:2271:18: 2271:28}> as Iterator>::collect` in `microkernel::Kernel::stride_snapshot`
callee `<synthyrion_abi::SysResult as PartialEq>::eq` in `microkernel::Kernel::syscall`
callee `Cap::object` in `microkernel::Kernel::syscall`
callee `authorize` in `microkernel::Kernel::syscall`
callee `<synthyrion_abi::SysResult as PartialEq>::ne` in `microkernel::Kernel::syscall_sealed`
callee `authorize_sealed` in `microkernel::Kernel::syscall_sealed`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as Iterator>::all` in `microkernel::Kernel::table_links_verified`
callee `<core::slice::Iter<'_, u64> as Iterator>::all` in `microkernel::Kernel::table_links_verified::{closure#0}`
callee `AddressSpace::is_poisoned` in `microkernel::Kernel::table_links_verified::{closure#0}`
callee `AddressSpace::table_frames` in `microkernel::Kernel::table_links_verified::{closure#0}`
callee `AddressSpace::verify_links` in `microkernel::Kernel::table_links_verified::{closure#0}`
callee `synthyrion_vmm::treeview::record_trusted` in `microkernel::Kernel::table_links_verified::{closure#0}`
callee `AddressSpace::translate` in `microkernel::Kernel::translate`
callee `RoView::health` in `microkernel::Kernel::translate`
callee `<Result<(), MemError> as Try>::branch` in `microkernel::Kernel::with_secret_and_memory_reserve`
callee `<Result<Kernel, MemError> as FromResidual<Result<Infallible, MemError>>>::from_residual` in `microkernel::Kernel::with_secret_and_memory_reserve`
callee `<[u8; 32] as Index<RangeFull>>::index` in `microkernel::VaultEvidence::bytes`
callee `<alloc::vec::Vec<u8> as Index<RangeFull>>::index` in `microkernel::VaultEvidence::bytes`
callee `<u16 as TryFrom<u64>>::try_from` in `microkernel::asid_rows::Kernel::serve_asid_rows`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::asid_rows::Kernel::serve_asid_rows`
callee `AddressSpace::set_hardware_entry` in `microkernel::asidcs::ArchHooks::write_image_half`
callee `AddressSpace::set_hardware_entry` in `microkernel::asidcs::ArchHooks::write_kernel_half`
callee `<Result<(), VsError> as FromResidual<Result<Infallible, VsError>>>::from_residual` in `microkernel::asidcs::Kernel::cs_asid_assign`
callee `<Result<(u64, u64), VsError> as Try>::branch` in `microkernel::asidcs::Kernel::cs_asid_assign`
callee `<Result<u64, VsError> as Try>::branch` in `microkernel::asidcs::Kernel::cs_asid_assign`
callee `Result::map_err VsError {VsError::Obj}>` in `microkernel::asidcs::Kernel::cs_asid_assign`
callee `<Result<(), VsError> as FromResidual<Result<Infallible, VsError>>>::from_residual` in `microkernel::asidcs::Kernel::cs_asid_pool_make`
callee `<Result<u64, VsError> as Try>::branch` in `microkernel::asidcs::Kernel::cs_asid_pool_make`
callee `core::ops::Range::contains` in `microkernel::asidcs::Kernel::cs_asid_pool_make`
callee `<Result<u64, VsError> as FromResidual<Result<Infallible, VsError>>>::from_residual` in `microkernel::asidcs::Kernel::cs_frame_flush`
callee `<Result<u64, VsError> as Try>::branch` in `microkernel::asidcs::Kernel::cs_frame_flush`
callee `<Result<(), VsError> as FromResidual<Result<Infallible, VsError>>>::from_residual` in `microkernel::asidcs::Kernel::cs_frame_map`
callee `<Result<(u64, u64), VsError> as Try>::branch` in `microkernel::asidcs::Kernel::cs_frame_map`
callee `<Result<u64, VsError> as Try>::branch` in `microkernel::asidcs::Kernel::cs_frame_map`
callee `Result::map_err VsError {VsError::Map}>` in `microkernel::asidcs::Kernel::cs_frame_map`
callee `<Result<(), VsError> as FromResidual<Result<Infallible, VsError>>>::from_residual` in `microkernel::asidcs::Kernel::cs_frame_unmap`
callee `<Result<(u64, u64), VsError> as Try>::branch` in `microkernel::asidcs::Kernel::cs_frame_unmap`
callee `<Result<(u64, u64, PageSize), VsError> as Try>::branch` in `microkernel::asidcs::Kernel::cs_frame_unmap`
callee `<Result<u64, VsError> as Try>::branch` in `microkernel::asidcs::Kernel::cs_frame_unmap`
callee `Result::map_err VsError {VsError::Map}>` in `microkernel::asidcs::Kernel::cs_frame_unmap`
callee `<alloc::vec::Vec<(u64, alloc::vec::Vec<(usize, u64)>)> as Deref>::deref` in `microkernel::asidcs::Kernel::dispatch_tables_ok`
callee `<core::slice::Iter<'_, (u64, alloc::vec::Vec<(usize, u64)>)> as Iterator>::all` in `microkernel::asidcs::Kernel::dispatch_tables_ok`
callee `Option::and_then u64, {closure@crates\microkernel\src\asidcs.rs:228:56: 228:59}>` in `microkernel::asidcs::Kernel::dispatch_tables_ok`
callee `synthyrion_vmm::treeview::apply_health` in `microkernel::asidcs::Kernel::dispatch_tables_ok::{closure#1}`
callee `<core::ops::Range<usize> as Iterator>::all` in `microkernel::asidcs::Kernel::dispatch_tables_ok::{closure#2}`
callee `<core::slice::Iter<'_, (usize, u64)> as Iterator>::find` in `microkernel::asidcs::Kernel::dispatch_tables_ok::{closure#2}::{closure#0}`
callee `move _4` in `microkernel::asidcs::Kernel::dispatch_tables_ok::{closure#2}::{closure#0}`
callee `<Filter<core::slice::Iter<'_, (u64, u64, alloc::vec::Vec<u64>)>, {closure@crates\microkernel\src\asidcs.rs:236:88: 236:99}> as Iterator>::all` in `microkernel::asidcs::Kernel::dispatch_tables_ok::{closure#3}`
callee `<alloc::vec::Vec<(u64, u64, alloc::vec::Vec<u64>)> as Deref>::deref` in `microkernel::asidcs::Kernel::dispatch_tables_ok::{closure#3}`
callee `<core::slice::Iter<'_, (u64, u64, alloc::vec::Vec<u64>)> as Iterator>::filter` in `microkernel::asidcs::Kernel::dispatch_tables_ok::{closure#3}`
callee `<Enumerate<core::slice::Iter<'_, u64>> as Iterator>::all` in `microkernel::asidcs::Kernel::dispatch_tables_ok::{closure#3}::{closure#1}`
callee `move _6` in `microkernel::asidcs::Kernel::dispatch_tables_ok::{closure#3}::{closure#1}::{closure#0}`
callee `<Result<u64, VsError> as FromResidual<Result<Infallible, VsError>>>::from_residual` in `microkernel::asidcs::Kernel::frame_flush_exec`
callee `<Result<u64, VsError> as Try>::branch` in `microkernel::asidcs::Kernel::frame_flush_exec`
callee `<Enumerate<core::slice::Iter<'_, (usize, u64)>> as Iterator>::any` in `microkernel::asidcs::Kernel::freeze_kernel_half`
callee `<core::slice::Iter<'_, (usize, u64)> as Iterator>::any` in `microkernel::asidcs::Kernel::freeze_kernel_half`
callee `<core::slice::Iter<'_, (usize, u64)> as Iterator>::enumerate` in `microkernel::asidcs::Kernel::freeze_kernel_half`
callee `Option::get_or_insert_with Box<ArchHooks> {<Box<ArchHooks> as Default>::default}>` in `microkernel::asidcs::Kernel::freeze_kernel_half`
callee `alloc::vec::Vec::extend_from_slice` in `microkernel::asidcs::Kernel::freeze_kernel_half`
callee `<core::slice::Iter<'_, (usize, u64)> as Iterator>::any` in `microkernel::asidcs::Kernel::freeze_kernel_half::{closure#1}::{closure#0}`
callee `<Copied<core::slice::Iter<'_, (usize, u64)>> as Iterator>::filter` in `microkernel::asidcs::Kernel::half_entries`
callee `<Filter<Copied<core::slice::Iter<'_, (usize, u64)>>, {closure@crates\microkernel\src\asidcs.rs:140:77: 140:85}> as Iterator>::collect` in `microkernel::asidcs::Kernel::half_entries`
callee `<Filter<core::slice::Iter<'_, (u64, usize, u64)>, {closure@crates\microkernel\src\asidcs.rs:141:45: 141:56}> as Iterator>::map` in `microkernel::asidcs::Kernel::half_entries`
callee `<alloc::vec::Vec<(usize, u64)> as DerefMut>::deref_mut` in `microkernel::asidcs::Kernel::half_entries`
callee `<alloc::vec::Vec<(usize, u64)> as Extend<(usize, u64)>>::extend` in `microkernel::asidcs::Kernel::half_entries`
callee `<core::slice::Iter<'_, (u64, usize, u64)> as Iterator>::filter` in `microkernel::asidcs::Kernel::half_entries`
callee `<core::slice::Iter<'_, (usize, u64)> as Iterator>::copied` in `microkernel::asidcs::Kernel::half_entries`
callee `core::slice::sort_unstable_by_key` in `microkernel::asidcs::Kernel::half_entries`
callee `<core::slice::Iter<'_, (u64, usize, u64)> as Iterator>::any` in `microkernel::asidcs::Kernel::half_entries::{closure#0}`
callee `<&usize as PartialEq>::eq` in `microkernel::asidcs::Kernel::half_entries::{closure#0}::{closure#0}`
callee `core::slice::contains` in `microkernel::asidcs::Kernel::is_charged::{closure#0}`
callee `<Result<synthyrion_vmm::cachemaint::FlushPlan, VsError> as Try>::branch` in `microkernel::asidcs::Kernel::run_flush`
callee `<Result<u64, VsError> as FromResidual<Result<Infallible, VsError>>>::from_residual` in `microkernel::asidcs::Kernel::run_flush`
callee `Option::and_then u64, {closure@crates\microkernel\src\asidcs.rs:384:56: 384:59}>` in `microkernel::asidcs::Kernel::run_flush`
callee `Result::map_err VsError {VsError::Flush}>` in `microkernel::asidcs::Kernel::run_flush`
callee `copy _20` in `microkernel::asidcs::Kernel::run_flush`
callee `<core::iter::Map<core::ops::Range<usize>, {closure@crates\microkernel\src\asidcs.rs:265:52: 265:55}> as Iterator>::collect` in `microkernel::asidcs::Kernel::seal_image_pages`
callee `<core::ops::Range<usize> as Iterator>::map` in `microkernel::asidcs::Kernel::seal_image_pages`
callee `<core::slice::Iter<'_, (u64, usize, u64)> as Iterator>::any` in `microkernel::asidcs::Kernel::seal_image_pages`
callee `alloc::vec::Vec::push` in `microkernel::asidcs::Kernel::seal_image_pages`
callee `alloc::vec::Vec::retain` in `microkernel::asidcs::Kernel::seal_image_pages`
loop `microkernel::asidcs::Kernel::seal_image_pages` header bb18
callee `move _3` in `microkernel::asidcs::Kernel::seal_image_pages::{closure#2}`
callee `Option::get_or_insert_with Box<ArchHooks> {<Box<ArchHooks> as Default>::default}>` in `microkernel::asidcs::Kernel::set_cache_executor`
callee `Option::get_or_insert_with Box<ArchHooks> {<Box<ArchHooks> as Default>::default}>` in `microkernel::asidcs::Kernel::set_space_executor`
callee `Option::get_or_insert_with Box<ArchHooks> {<Box<ArchHooks> as Default>::default}>` in `microkernel::asidcs::Kernel::set_table_hooks`
callee `Rights::contains` in `microkernel::asidcs::Kernel::vs_mem`
callee `Rights::contains` in `microkernel::asidcs::Kernel::vs_task_space`
callee `<Option<fn(Option<u64>)> as Default>::default` in `microkernel::asidcs::default`
callee `<Option<fn(u64, u64, bool) -> u64> as Default>::default` in `microkernel::asidcs::default`
callee `<Option<fn(u64, usize) -> u64> as Default>::default` in `microkernel::asidcs::default`
callee `<alloc::vec::Vec<(u64, u64, alloc::vec::Vec<u64>)> as Default>::default` in `microkernel::asidcs::default`
callee `<alloc::vec::Vec<(u64, usize, u64)> as Default>::default` in `microkernel::asidcs::default`
callee `<alloc::vec::Vec<(usize, u64)> as Default>::default` in `microkernel::asidcs::default`
callee `<alloc::vec::Vec<u64> as Default>::default` in `microkernel::asidcs::default`
callee `<bool as Default>::default` in `microkernel::asidcs::default`
callee `<u64 as Default>::default` in `microkernel::asidcs::default`
callee `<&cspace::CsError as PartialEq>::eq` in `microkernel::asidcs::eq`
callee `<&objects::FlushError as PartialEq>::eq` in `microkernel::asidcs::eq`
callee `<&objects::MapError as PartialEq>::eq` in `microkernel::asidcs::eq`
callee `<&objects::ObjError as PartialEq>::eq` in `microkernel::asidcs::eq`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::asidcs::fmt`
loop `microkernel::asidobj::AsidTable::alloc_range` header bb9
callee `<u16 as TryFrom<u32>>::try_from` in `microkernel::asidobj::AsidTable::alloc_range::{closure#0}`
callee `<SpaceAsid as Default>::default` in `microkernel::asidobj::AsidTable::assign`
loop `microkernel::asidobj::AsidTable::clear_control` header bb4
loop `microkernel::asidobj::AsidTable::clear_control` header bb10
loop `microkernel::asidobj::AsidTable::clear_control` header bb15
loop `microkernel::asidobj::AsidTable::drop_space` header bb5
callee `<core::ops::Range<usize> as Iterator>::find` in `microkernel::asidobj::AsidTable::find_pool`
callee `<core::ops::Range<usize> as Iterator>::find` in `microkernel::asidobj::AsidTable::find_space`
callee `<Option<SpaceAsid> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::asidobj::AsidTable::global`
callee `core::slice::contains` in `microkernel::asidobj::AsidTable::note_unmap`
loop `microkernel::asidobj::AsidTable::note_unmap` header bb10
callee `<Option<SpaceAsid> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::asidobj::AsidTable::of_space`
callee `<core::ops::Range<usize> as Iterator>::any` in `microkernel::asidobj::AsidTable::range_free`
loop `microkernel::asidobj::AsidTable::remove_pool` header bb5
loop `microkernel::asidobj::AsidTable::remove_pool` header bb16
loop `microkernel::asidobj::AsidTable::remove_pool` header bb21
loop `microkernel::asidobj::AsidTable::remove_pool` header bb31
callee `<Result<SwitchKind, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::asidobj::AsidTable::switch`
callee `<Result<SwitchKind, objects::ObjError> as Try>::branch` in `microkernel::asidobj::AsidTable::switch`
callee `AsidPool::switch_to` in `microkernel::asidobj::AsidTable::switch`
loop `microkernel::asidobj::AsidTable::take` header bb4
loop `microkernel::asidobj::AsidTable::take` header bb15
callee `<AsidPool as Clone>::clone` in `microkernel::asidobj::clone`
callee `<CoreTlbs as Clone>::clone` in `microkernel::asidobj::clone`
callee `<Option<u16> as Clone>::clone` in `microkernel::asidobj::clone`
callee `<Option<u64> as Clone>::clone` in `microkernel::asidobj::clone`
callee `<SpaceAsid as Clone>::clone` in `microkernel::asidobj::clone`
callee `<CoreTlbs as Default>::default` in `microkernel::asidobj::default`
callee `<Option<u16> as Default>::default` in `microkernel::asidobj::default`
callee `<Option<u64> as Default>::default` in `microkernel::asidobj::default`
callee `<bool as Default>::default` in `microkernel::asidobj::default`
callee `<u16 as Default>::default` in `microkernel::asidobj::default`
callee `<u64 as Default>::default` in `microkernel::asidobj::default`
callee `<usize as Default>::default` in `microkernel::asidobj::default`
callee `Formatter::debug_struct_field2_finish` in `microkernel::asidobj::fmt`
callee `Formatter::debug_struct_field3_finish` in `microkernel::asidobj::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::asidobj::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::asidobj::fmt`
loop `microkernel::bidx::BoundedIdx::insert` header bb12
callee `<Zip<core::slice::Iter<'_, u64>, core::slice::Iter<'_, V>> as Iterator>::take` in `microkernel::bidx::BoundedIdx::iter`
loop `microkernel::bidx::BoundedIdx::remove` header bb8
callee `<core::slice::Iter<'_, V> as Iterator>::take` in `microkernel::bidx::BoundedIdx::values`
callee `<core::slice::IterMut<'_, V> as Iterator>::take` in `microkernel::bidx::BoundedIdx::values_mut`
callee `alloc::vec::Vec::into_boxed_slice` in `microkernel::bidx::BoundedIdx::with_capacity`
callee `alloc::vec::from_elem` in `microkernel::bidx::BoundedIdx::with_capacity`
callee `<V as Default>::default` in `microkernel::bidx::default`
callee `<dyn bindings::VaultService as bindings::VaultService>::grant` in `microkernel::bindings::Kernel::grant_vault`
callee `and_then (closure bindings::Kernel::last_vault_evidence::{closure#0} is itself unbounded)` in `microkernel::bindings::Kernel::last_vault_evidence`
callee `and_then (closure bindings::Kernel::last_vault_tag::{closure#0} is itself unbounded)` in `microkernel::bindings::Kernel::last_vault_tag`
callee `<dyn bindings::VaultService as bindings::VaultService>::revoke_all` in `microkernel::bindings::Kernel::revoke_vault_caps::{closure#0}`
callee `<[u8; 40] as IndexMut<RangeTo<usize>>>::index_mut` in `microkernel::bindings::Kernel::vault_op_inner`
callee `<dyn bindings::VaultService as bindings::VaultService>::operate` in `microkernel::bindings::Kernel::vault_op_inner`
callee `core::slice::copy_from_slice` in `microkernel::bindings::Kernel::vault_op_inner`
loop `microkernel::bindings::Kernel::vault_op_inner` header bb10
callee `<&VaultEvidence as PartialEq>::eq` in `microkernel::bindings::eq`
callee `Formatter::debug_struct_field4_finish` in `microkernel::bindings::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::bindings::fmt`
callee `Formatter::write_str` in `microkernel::bindings::fmt`
callee `<Result<(), BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::decode`
callee `<Result<BootInfo, BootInfoError> as FromResidual<Result<Infallible, BootInfoError>>>::from_residual` in `microkernel::bootinfo::BootInfo::decode`
callee `<Result<alloc::vec::Vec<DeviceDesc>, BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::decode`
callee `<Result<alloc::vec::Vec<UntypedDesc>, BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::decode`
callee `<Result<u32, BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::decode`
callee `<Result<u8, BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::decode`
callee `<Result<usize, BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::decode`
callee `<core::iter::Map<core::ops::Range<usize>, {closure@crates\microkernel\src\bootinfo.rs:287:22: 287:25}> as Iterator>::collect` in `microkernel::bootinfo::BootInfo::decode`
callee `<core::iter::Map<core::ops::Range<usize>, {closure@crates\microkernel\src\bootinfo.rs:290:22: 290:25}> as Iterator>::collect` in `microkernel::bootinfo::BootInfo::decode`
callee `<core::iter::Map<core::ops::Range<usize>, {closure@crates\microkernel\src\bootinfo.rs:297:22: 297:25}> as Iterator>::next` in `microkernel::bootinfo::BootInfo::decode`
callee `<core::ops::Range<usize> as Iterator>::map` in `microkernel::bootinfo::BootInfo::decode`
callee `core::num::is_multiple_of` in `microkernel::bootinfo::BootInfo::decode`
callee `<&[u8] as TryInto<[u8; 8]>>::try_into` in `microkernel::bootinfo::BootInfo::decode::{closure#0}`
callee `core::num::from_le_bytes` in `microkernel::bootinfo::BootInfo::decode::{closure#0}`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::bootinfo::BootInfo::decode::{closure#2}`
callee `<Result<UntypedDesc, BootInfoError> as FromResidual<Result<Infallible, BootInfoError>>>::from_residual` in `microkernel::bootinfo::BootInfo::decode::{closure#4}`
callee `<Result<usize, BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::decode::{closure#4}`
callee `call (2 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::bootinfo::BootInfo::decode::{closure#4}`
callee `<Result<DeviceDesc, BootInfoError> as FromResidual<Result<Infallible, BootInfoError>>>::from_residual` in `microkernel::bootinfo::BootInfo::decode::{closure#5}`
callee `<Result<u32, BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::decode::{closure#5}`
callee `<Result<usize, BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::decode::{closure#5}`
callee `call (3 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::bootinfo::BootInfo::decode::{closure#5}`
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::bootinfo::BootInfo::decode::{closure#6}`
callee `<&alloc::vec::Vec<DeviceDesc> as IntoIterator>::into_iter` in `microkernel::bootinfo::BootInfo::encode`
callee `<&alloc::vec::Vec<UntypedDesc> as IntoIterator>::into_iter` in `microkernel::bootinfo::BootInfo::encode`
callee `<FlatMap<core::slice::Iter<'_, u64>, [u8; 8], {closure@crates\microkernel\src\bootinfo.rs:253:27: 253:30}> as Iterator>::collect` in `microkernel::bootinfo::BootInfo::encode`
callee `<core::slice::Iter<'_, DeviceDesc> as Iterator>::next` in `microkernel::bootinfo::BootInfo::encode`
callee `<core::slice::Iter<'_, UntypedDesc> as Iterator>::next` in `microkernel::bootinfo::BootInfo::encode`
callee `<core::slice::Iter<'_, u64> as Iterator>::flat_map` in `microkernel::bootinfo::BootInfo::encode`
callee `alloc::vec::Vec::extend_from_slice` in `microkernel::bootinfo::BootInfo::encode`
callee `alloc::vec::Vec::with_capacity` in `microkernel::bootinfo::BootInfo::encode`
loop `microkernel::bootinfo::BootInfo::encode` header bb14
loop `microkernel::bootinfo::BootInfo::encode` header bb20
callee `<&alloc::vec::Vec<DeviceDesc> as IntoIterator>::into_iter` in `microkernel::bootinfo::BootInfo::validate`
callee `<&alloc::vec::Vec<UntypedDesc> as IntoIterator>::into_iter` in `microkernel::bootinfo::BootInfo::validate`
callee `<Enumerate<core::slice::Iter<'_, UntypedDesc>> as IntoIterator>::into_iter` in `microkernel::bootinfo::BootInfo::validate`
callee `<Enumerate<core::slice::Iter<'_, UntypedDesc>> as Iterator>::next` in `microkernel::bootinfo::BootInfo::validate`
callee `<Result<(), BootInfoError> as FromResidual<Result<Infallible, BootInfoError>>>::from_residual` in `microkernel::bootinfo::BootInfo::validate`
callee `<Result<u64, BootInfoError> as Try>::branch` in `microkernel::bootinfo::BootInfo::validate`
callee `<[u64; 8] as PartialEq>::eq` in `microkernel::bootinfo::BootInfo::validate`
callee `<alloc::vec::Vec<UntypedDesc> as Deref>::deref` in `microkernel::bootinfo::BootInfo::validate`
callee `<core::slice::Iter<'_, DeviceDesc> as Iterator>::next` in `microkernel::bootinfo::BootInfo::validate`
callee `<core::slice::Iter<'_, UntypedDesc> as Iterator>::enumerate` in `microkernel::bootinfo::BootInfo::validate`
callee `<core::slice::Iter<'_, UntypedDesc> as Iterator>::next` in `microkernel::bootinfo::BootInfo::validate`
loop `microkernel::bootinfo::BootInfo::validate` header bb12
loop `microkernel::bootinfo::BootInfo::validate` header bb22
loop `microkernel::bootinfo::BootInfo::validate` header bb48
callee `<Result<(), BootError> as FromResidual<Result<Infallible, BootError>>>::from_residual` in `microkernel::bootinfo::Kernel::boot_endpoint_pair`
callee `<Result<(), BootError> as Try>::branch` in `microkernel::bootinfo::Kernel::boot_endpoint_pair`
loop `microkernel::bootinfo::Kernel::boot_readback` header bb11
callee `<&[BootDevice] as IntoIterator>::into_iter` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<&[usize] as IntoIterator>::into_iter` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<&alloc::vec::Vec<synthyrion_capauth::SealedCap> as IntoIterator>::into_iter` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<BootInfo as Clone>::clone` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<Chain<core::slice::Iter<'_, synthyrion_capauth::SealedCap>, core::option::Iter<'_, synthyrion_capauth::SealedCap>> as IntoIterator>::into_iter` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<Chain<core::slice::Iter<'_, synthyrion_capauth::SealedCap>, core::option::Iter<'_, synthyrion_capauth::SealedCap>> as Iterator>::next` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<Enumerate<core::slice::Iter<'_, BootDevice>> as IntoIterator>::into_iter` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<Enumerate<core::slice::Iter<'_, BootDevice>> as Iterator>::next` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<Enumerate<core::slice::Iter<'_, synthyrion_capauth::SealedCap>> as IntoIterator>::into_iter` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<Enumerate<core::slice::Iter<'_, synthyrion_capauth::SealedCap>> as Iterator>::next` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<[BootDevice] as Index<RangeTo<usize>>>::index` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<alloc::vec::Vec<synthyrion_capauth::SealedCap> as Deref>::deref` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<alloc::vec::Vec<synthyrion_capauth::SealedCap> as DerefMut>::deref_mut` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<core::slice::Iter<'_, BootDevice> as Iterator>::any` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<core::slice::Iter<'_, BootDevice> as Iterator>::enumerate` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<core::slice::Iter<'_, BootDevice> as Iterator>::next` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<core::slice::Iter<'_, synthyrion_capauth::SealedCap> as Iterator>::chain` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<core::slice::Iter<'_, synthyrion_capauth::SealedCap> as Iterator>::enumerate` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<core::slice::Iter<'_, synthyrion_capauth::SealedCap> as Iterator>::next` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `<core::slice::Iter<'_, usize> as Iterator>::next` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `BTreeSet::insert` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `Box::new` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `Option::iter` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `alloc::slice::sort_by_key` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `alloc::vec::Vec::push` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `and_then (closure bootinfo::Kernel::boot_root_task::{closure#1} is itself unbounded)` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `and_then (closure bootinfo::Kernel::boot_root_task::{closure#2} is itself unbounded)` in `microkernel::bootinfo::Kernel::boot_root_task`
callee `core::slice::contains` in `microkernel::bootinfo::Kernel::boot_root_task`
loop `microkernel::bootinfo::Kernel::boot_root_task` header bb22
loop `microkernel::bootinfo::Kernel::boot_root_task` header bb42
loop `microkernel::bootinfo::Kernel::boot_root_task` header bb60
loop `microkernel::bootinfo::Kernel::boot_root_task` header bb82
loop `microkernel::bootinfo::Kernel::boot_root_task` header bb93
loop `microkernel::bootinfo::Kernel::boot_root_task` header bb125
loop `microkernel::bootinfo::Kernel::boot_root_task` header bb144
loop `microkernel::bootinfo::Kernel::boot_root_task` header bb186
callee `alloc::vec::Vec::pop` in `microkernel::bootinfo::Kernel::boot_root_task::{closure#2}`
callee `<[u64; 8] as PartialEq>::eq` in `microkernel::bootinfo::Kernel::set_boot_image`
callee `<BootInfo as Clone>::clone` in `microkernel::bootinfo::clone`
callee `<ImageDesc as Clone>::clone` in `microkernel::bootinfo::clone`
callee `<synthyrion_capauth::SealedCap as Clone>::clone` in `microkernel::bootinfo::clone`
callee `<alloc::vec::Vec<DeviceDesc> as Clone>::clone` in `microkernel::bootinfo::clone`
callee `<alloc::vec::Vec<UntypedDesc> as Clone>::clone` in `microkernel::bootinfo::clone`
callee `<alloc::vec::Vec<synthyrion_capauth::SealedCap> as Clone>::clone` in `microkernel::bootinfo::clone`
callee `<Option<Box<BootInfo>> as Default>::default` in `microkernel::bootinfo::default`
callee `<Option<Box<DriverReports>> as Default>::default` in `microkernel::bootinfo::default`
callee `<u32 as Default>::default` in `microkernel::bootinfo::default`
callee `<ImageDesc as PartialEq>::eq` in `microkernel::bootinfo::eq`
callee `<[u64; 8] as PartialEq>::eq` in `microkernel::bootinfo::eq`
callee `<synthyrion_caps::Rights as PartialEq>::eq` in `microkernel::bootinfo::eq`
callee `<alloc::vec::Vec<DeviceDesc> as PartialEq>::eq` in `microkernel::bootinfo::eq`
callee `<alloc::vec::Vec<UntypedDesc> as PartialEq>::eq` in `microkernel::bootinfo::eq`
callee `Formatter::debug_struct_field2_finish` in `microkernel::bootinfo::fmt`
callee `Formatter::debug_struct_field3_finish` in `microkernel::bootinfo::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::bootinfo::fmt`
callee `Formatter::write_str` in `microkernel::bootinfo::fmt`
callee `<Enumerate<core::slice::Iter<'_, u32>> as IntoIterator>::into_iter` in `microkernel::bornglue::KRing::ring_rebuild`
callee `<Enumerate<core::slice::Iter<'_, u32>> as Iterator>::next` in `microkernel::bornglue::KRing::ring_rebuild`
callee `<core::slice::Iter<'_, u32> as Iterator>::enumerate` in `microkernel::bornglue::KRing::ring_rebuild`
loop `microkernel::bornglue::KRing::ring_rebuild` header bb10
callee `<dyn FnMut(u32) -> bool as FnMut<(u32,)>>::call_mut` in `microkernel::bornglue::KRing::ring_walk`
loop `microkernel::bornglue::KRing::ring_walk` header bb7
callee `<&BTreeSet<u64> as IntoIterator>::into_iter` in `microkernel::bornglue::Kernel::born_audit`
callee `<&alloc::collections::BTreeMap<u64, BTreeSet<u64>> as IntoIterator>::into_iter` in `microkernel::bornglue::Kernel::born_audit`
callee `<BTreeSet<u32> as PartialEq>::ne` in `microkernel::bornglue::Kernel::born_audit`
callee `<Copied<core::slice::Iter<'_, u32>> as Iterator>::collect` in `microkernel::bornglue::Kernel::born_audit`
callee `<FilterMap<alloc::collections::btree_set::Iter<'_, u64>, {closure@crates\microkernel\src\cspace\bornglue.rs:202:61: 202:64}> as Iterator>::collect` in `microkernel::bornglue::Kernel::born_audit`
callee `<alloc::collections::btree_map::Iter<'_, u64, BTreeSet<u64>> as Iterator>::next` in `microkernel::bornglue::Kernel::born_audit`
callee `<alloc::collections::btree_set::Iter<'_, u64> as Iterator>::filter_map` in `microkernel::bornglue::Kernel::born_audit`
callee `<alloc::collections::btree_set::Iter<'_, u64> as Iterator>::next` in `microkernel::bornglue::Kernel::born_audit`
callee `<core::slice::Iter<'_, u32> as Iterator>::copied` in `microkernel::bornglue::Kernel::born_audit`
callee `Arguments::new` in `microkernel::bornglue::Kernel::born_audit`
callee `BTreeSet::iter` in `microkernel::bornglue::Kernel::born_audit`
callee `alloc::collections::BTreeMap::contains_key` in `microkernel::bornglue::Kernel::born_audit`
callee `alloc::collections::BTreeMap::insert` in `microkernel::bornglue::Kernel::born_audit`
callee `alloc::fmt::format` in `microkernel::bornglue::Kernel::born_audit`
callee `core::fmt::rt::Argument::new_debug` in `microkernel::bornglue::Kernel::born_audit`
callee `core::fmt::rt::Argument::new_lower_hex` in `microkernel::bornglue::Kernel::born_audit`
callee `must_use` in `microkernel::bornglue::Kernel::born_audit`
loop `microkernel::bornglue::Kernel::born_audit` header bb3
loop `microkernel::bornglue::Kernel::born_audit` header bb15
loop `microkernel::bornglue::Kernel::born_audit` header bb49
callee `alloc::collections::BTreeMap::get` in `microkernel::bornglue::Kernel::born_audit::{closure#0}`
callee `alloc::vec::Vec::push` in `microkernel::bornglue::Kernel::born_audit::{closure#0}`
loop `microkernel::bornglue::Kernel::born_cells_free` header bb3
callee `<Enumerate<core::slice::Iter<'_, u32>> as IntoIterator>::into_iter` in `microkernel::bornglue::RingRead::ring_rebuild`
callee `<Enumerate<core::slice::Iter<'_, u32>> as Iterator>::next` in `microkernel::bornglue::RingRead::ring_rebuild`
callee `<core::slice::Iter<'_, u32> as Iterator>::enumerate` in `microkernel::bornglue::RingRead::ring_rebuild`
loop `microkernel::bornglue::RingRead::ring_rebuild` header bb10
callee `<dyn FnMut(u32) -> bool as FnMut<(u32,)>>::call_mut` in `microkernel::bornglue::RingRead::ring_walk`
loop `microkernel::bornglue::RingRead::ring_walk` header bb7
callee `<&u64 as PartialEq>::eq` in `microkernel::bornglue::eq`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::bornglue::fmt`
callee `Formatter::write_str` in `microkernel::bornglue::fmt`
callee `<&u32 as PartialEq>::eq` in `microkernel::bornring::eq`
callee `<&usize as PartialEq>::eq` in `microkernel::bornring::eq`
callee `Formatter::debug_struct_field2_finish` in `microkernel::bornring::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::bornring::fmt`
callee `Formatter::write_str` in `microkernel::bornring::fmt`
loop `microkernel::capcontain::Kernel::drain_deferred` header bb2
callee `<GuardedArr<64, u64::MAX> as Clone>::clone` in `microkernel::capcontain::clone`
callee `<Option<u64> as Default>::default` in `microkernel::capcontain::default`
callee `<usize as Default>::default` in `microkernel::capcontain::default`
callee `GuardedArr::new` in `microkernel::capcontain::default`
callee `Formatter::debug_struct_field2_finish` in `microkernel::capcontain::fmt`
callee `Formatter::debug_struct_field3_finish` in `microkernel::capcontain::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::capcontain::fmt`
callee `Formatter::write_str` in `microkernel::capcontain::fmt`
callee `<Option<&Dual> as Try>::branch` in `microkernel::capguard::BornMap::get`
callee `<Option<&alloc::vec::Vec<u64>> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::capguard::BornMap::get`
callee `alloc::collections::BTreeMap::get` in `microkernel::capguard::BornMap::get`
callee `alloc::collections::BTreeMap::get_mut` in `microkernel::capguard::BornMap::heal_entry`
callee `Box::new_uninit` in `microkernel::capguard::BornMap::insert_member`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `microkernel::capguard::BornMap::insert_member`
callee `alloc::collections::BTreeMap::get_mut` in `microkernel::capguard::BornMap::insert_member`
callee `alloc::collections::BTreeMap::insert` in `microkernel::capguard::BornMap::insert_member`
callee `alloc::vec::Vec::insert` in `microkernel::capguard::BornMap::insert_member`
callee `core::slice::binary_search` in `microkernel::capguard::BornMap::insert_member`
callee `<alloc::collections::btree_map::Range<'_, u64, Dual> as Iterator>::next` in `microkernel::capguard::BornMap::next_id`
callee `alloc::collections::BTreeMap::range` in `microkernel::capguard::BornMap::next_id`
callee `<Filter<alloc::collections::btree_map::Iter<'_, u64, Dual>, {closure@crates\microkernel\src\cspace\capguard.rs:214:30: 214:38}> as Iterator>::map` in `microkernel::capguard::BornMap::objects_of`
callee `<alloc::collections::btree_map::Iter<'_, u64, Dual> as Iterator>::filter` in `microkernel::capguard::BornMap::objects_of`
callee `<core::iter::Map<Filter<alloc::collections::btree_map::Iter<'_, u64, Dual>, {closure@crates\microkernel\src\cspace\capguard.rs:214:30: 214:38}>, {closure@crates\microkernel\src\cspace\capguard.rs:214:88: 214:96}> as Iterator>::collect` in `microkernel::capguard::BornMap::objects_of`
callee `alloc::collections::BTreeMap::iter` in `microkernel::capguard::BornMap::objects_of`
callee `core::slice::binary_search` in `microkernel::capguard::BornMap::objects_of::{closure#0}`
callee `alloc::collections::BTreeMap::remove` in `microkernel::capguard::BornMap::remove`
callee `<Option<&mut Dual> as Try>::branch` in `microkernel::capguard::BornMap::remove_member`
callee `alloc::collections::BTreeMap::get_mut` in `microkernel::capguard::BornMap::remove_member`
callee `alloc::vec::Vec::remove` in `microkernel::capguard::BornMap::remove_member`
callee `core::slice::binary_search` in `microkernel::capguard::BornMap::remove_member`
callee `alloc::collections::BTreeMap::get_mut` in `microkernel::capguard::BornMap::set_from_mirror`
callee `<alloc::collections::btree_map::Iter<'_, u64, Dual> as Iterator>::map` in `microkernel::capguard::BornMap::to_map`
callee `<core::iter::Map<alloc::collections::btree_map::Iter<'_, u64, Dual>, {closure@crates\microkernel\src\cspace\capguard.rs:219:27: 219:35}> as Iterator>::collect` in `microkernel::capguard::BornMap::to_map`
callee `alloc::collections::BTreeMap::iter` in `microkernel::capguard::BornMap::to_map`
callee `<Copied<core::slice::Iter<'_, u64>> as Iterator>::collect` in `microkernel::capguard::BornMap::to_map::{closure#0}`
callee `<core::slice::Iter<'_, u64> as Iterator>::copied` in `microkernel::capguard::BornMap::to_map::{closure#0}`
callee `<core::ops::Range<usize> as Iterator>::all` in `microkernel::capguard::PendingStack::agrees`
callee `<RecHeal as PartialEq>::eq` in `microkernel::capguard::PendingStack::pop`
callee `<alloc::vec::Vec<(cspace::Work, Guarded<3, u64::MAX>)> as Deref>::deref` in `microkernel::capguard::PendingStack::pop`
callee `Option::is_some_and bool {PendingStack::agrees}>` in `microkernel::capguard::PendingStack::pop`
callee `alloc::vec::Vec::pop` in `microkernel::capguard::PendingStack::pop`
callee `alloc::vec::Vec::push` in `microkernel::capguard::PendingStack::push`
callee `<Option<&Dual> as Try>::branch` in `microkernel::capguard::RootMap::get`
callee `<Option<&u64> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::capguard::RootMap::get`
callee `alloc::collections::BTreeMap::get` in `microkernel::capguard::RootMap::get`
callee `alloc::collections::BTreeMap::get_mut` in `microkernel::capguard::RootMap::heal_entry`
callee `Box::new_uninit` in `microkernel::capguard::RootMap::insert`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `microkernel::capguard::RootMap::insert`
callee `alloc::collections::BTreeMap::insert` in `microkernel::capguard::RootMap::insert`
callee `<alloc::collections::btree_map::Range<'_, u64, Dual> as Iterator>::next` in `microkernel::capguard::RootMap::next_id`
callee `alloc::collections::BTreeMap::range` in `microkernel::capguard::RootMap::next_id`
callee `alloc::collections::BTreeMap::remove` in `microkernel::capguard::RootMap::remove_unchecked`
callee `alloc::collections::BTreeMap::retain` in `microkernel::capguard::RootMap::retain`
callee `<impl FnMut(&u64, &u64) -> bool as FnMut<(&u64, &u64)>>::call_mut` in `microkernel::capguard::RootMap::retain::{closure#0}::{closure#0}`
callee `<Cell<bool> as Clone>::clone` in `microkernel::capguard::clone`
callee `<Cell<u64> as Clone>::clone` in `microkernel::capguard::clone`
callee `<alloc::collections::BTreeMap<u64, Dual> as Clone>::clone` in `microkernel::capguard::clone`
callee `<alloc::vec::Vec<(cspace::Work, Guarded<3, u64::MAX>)> as Clone>::clone` in `microkernel::capguard::clone`
callee `<Cell<bool> as Default>::default` in `microkernel::capguard::default`
callee `<Cell<u64> as Default>::default` in `microkernel::capguard::default`
callee `<alloc::collections::BTreeMap<u64, Dual> as Default>::default` in `microkernel::capguard::default`
callee `<alloc::vec::Vec<(cspace::Work, Guarded<3, u64::MAX>)> as Default>::default` in `microkernel::capguard::default`
callee `<u64 as Default>::default` in `microkernel::capguard::default`
callee `Formatter::debug_struct_field2_finish` in `microkernel::capguard::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::capguard::fmt`
callee `<Copied<core::slice::Iter<'_, u64>> as Iterator>::min` in `microkernel::capheal::CapCursor::passes`
callee `<core::slice::Iter<'_, u64> as Iterator>::copied` in `microkernel::capheal::CapCursor::passes`
callee `<CapReport as Default>::default` in `microkernel::capheal::Kernel::delete_node_dying`
callee `<FaultLog<64> as Default>::default` in `microkernel::capheal::Kernel::delete_node_dying`
loop `microkernel::capheal::Kernel::delete_node_dying` header bb6
callee `alloc::vec::Vec::push` in `microkernel::capheal::Kernel::records_naming_object`
loop `microkernel::capheal::Kernel::records_naming_object` header bb2
callee `<ScrubStats as Default>::default` in `microkernel::capheal::default`
callee `<[u64; 5] as Default>::default` in `microkernel::capheal::default`
callee `<u64 as Default>::default` in `microkernel::capheal::default`
callee `<usize as Default>::default` in `microkernel::capheal::default`
callee `Formatter::debug_struct_field3_finish` in `microkernel::capheal::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::capheal::fmt`
callee `<&usize as PartialEq>::eq` in `microkernel::caprec::eq`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::caprec::fmt`
callee `Formatter::write_str` in `microkernel::caprec::fmt`
callee `Cell::new` in `microkernel::chainstore::ChainStore::new`
callee `Formatter::debug_struct_field4_finish` in `microkernel::chainstore::fmt`
callee `Formatter::write_str` in `microkernel::chainstore::fmt`
loop `microkernel::children_into` header bb3
callee `<linktree::Links as Default>::default` in `microkernel::clear`
callee `alloc::vec::from_elem` in `microkernel::cnfree::CnFree::new`
callee `<alloc::vec::Vec<u16> as Clone>::clone` in `microkernel::cnfree::clone`
callee `Formatter::debug_struct_field3_finish` in `microkernel::cnfree::fmt`
callee `core::num::div_ceil` in `microkernel::cnode_frames`
callee `<Zip<core::slice::IterMut<'_, u64>, core::iter::FromFn<{closure@crates\microkernel\src\gate\tcb_rows.rs:36:25: 36:32}>> as IntoIterator>::into_iter` in `microkernel::cnode_rows::Kernel::serve_cnode_rows`
callee `<Zip<core::slice::IterMut<'_, u64>, core::iter::FromFn<{closure@crates\microkernel\src\gate\tcb_rows.rs:36:25: 36:32}>> as Iterator>::next` in `microkernel::cnode_rows::Kernel::serve_cnode_rows`
loop `microkernel::cnode_rows::Kernel::serve_cnode_rows` header bb35
callee `<Result<alloc::vec::Vec<Option<SlotView>>, cspace::CsError> as FromResidual<Result<Infallible, cspace::CsError>>>::from_residual` in `microkernel::cspace::Kernel::cnode_dump`
callee `alloc::vec::Vec::push` in `microkernel::cspace::Kernel::cnode_dump`
callee `and_then (closure cspace::Kernel::cnode_dump::{closure#1} is itself unbounded)` in `microkernel::cspace::Kernel::cnode_dump`
loop `microkernel::cspace::Kernel::cnode_dump` header bb10
callee `and_then (closure cspace::Kernel::cnode_lookup::{closure#0} is itself unbounded)` in `microkernel::cspace::Kernel::cnode_lookup`
callee `<u16 as From<u8>>::from` in `microkernel::cspace::Kernel::cnode_set_guard`
callee `<Result<(), cspace::SlotRetypeError> as FromResidual<Result<Infallible, cspace::SlotRetypeError>>>::from_residual` in `microkernel::cspace::Kernel::cs_bind_sched_context`
callee `<Result<(synthyrion_caps::ObjKind, u64, synthyrion_caps::Rights, u64), cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_bind_sched_context`
callee `<Result<u64, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_bind_sched_context`
callee `Result::map_err cspace::SlotRetypeError {cspace::SlotRetypeError::Cs}>` in `microkernel::cspace::Kernel::cs_bind_sched_context`
callee `Result::map_err cspace::SlotRetypeError {cspace::SlotRetypeError::Obj}>` in `microkernel::cspace::Kernel::cs_bind_sched_context`
callee `Rights::contains` in `microkernel::cspace::Kernel::cs_bind_sched_context`
callee `<Result<(synthyrion_caps::ObjKind, u64, synthyrion_caps::Rights, u64), cspace::CsError> as FromResidual<Result<Infallible, cspace::CsError>>>::from_residual` in `microkernel::cspace::Kernel::cs_cap`
callee `<Result<(), cspace::SlotRetypeError> as FromResidual<Result<Infallible, cspace::SlotRetypeError>>>::from_residual` in `microkernel::cspace::Kernel::cs_configure_sched_context`
callee `<Result<u64, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_configure_sched_context`
callee `Result::map_err cspace::SlotRetypeError {cspace::SlotRetypeError::Obj}>` in `microkernel::cspace::Kernel::cs_configure_sched_context`
callee `<Result<(synthyrion_caps::ObjKind, u64, synthyrion_caps::Rights, u64), cspace::CsError> as Try>::branch` in `microkernel::cspace::Kernel::cs_irq_handler_get`
callee `Rights::contains` in `microkernel::cspace::Kernel::cs_irq_handler_get`
callee `and_then (closure cspace::Kernel::cs_lookup::{closure#0} is itself unbounded)` in `microkernel::cspace::Kernel::cs_lookup`
callee `<Result<(synthyrion_caps::ObjKind, u64, synthyrion_caps::Rights, u64), objects::MapError> as Try>::branch` in `microkernel::cspace::Kernel::cs_map_anonymous`
callee `Rights::contains` in `microkernel::cspace::Kernel::cs_map_anonymous`
callee `<Result<(u64, usize, synthyrion_caps::Rights), cspace::CsError> as Try>::branch` in `microkernel::cspace::Kernel::cs_resolve_need`
callee `Result::map_err cspace::CsError {cspace::CsError::Lookup}>` in `microkernel::cspace::Kernel::cs_resolve_need`
callee `Rights::contains` in `microkernel::cspace::Kernel::cs_resolve_need`
callee `<&alloc::vec::Vec<(u64, u64)> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<&alloc::vec::Vec<Made> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<CapNode as Clone>::clone` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Enumerate<core::slice::Iter<'_, Made>> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Enumerate<core::slice::Iter<'_, Made>> as Iterator>::next` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Filter<core::slice::Iter<'_, Made>, {closure@crates\microkernel\src\cspace.rs:1720:41: 1720:44}> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Filter<core::slice::Iter<'_, Made>, {closure@crates\microkernel\src\cspace.rs:1720:41: 1720:44}> as Iterator>::next` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Filter<core::slice::Iter<'_, Made>, {closure@crates\microkernel\src\cspace.rs:1725:41: 1725:44}> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Filter<core::slice::Iter<'_, Made>, {closure@crates\microkernel\src\cspace.rs:1725:41: 1725:44}> as Iterator>::next` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Result<(), cspace::SlotRetypeError> as FromResidual<Result<Infallible, cspace::SlotRetypeError>>>::from_residual` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Result<(), cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Result<(u64, usize), cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Result<CapNode, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Result<Option<u64>, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Result<alloc::vec::Vec<Made>, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Result<u64, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Result<usize, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Rev<core::slice::Iter<'_, (u64, u64)>> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<Rev<core::slice::Iter<'_, (u64, u64)>> as Iterator>::next` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<alloc::vec::Vec<Made> as Deref>::deref` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::any` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::next` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::rev` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<core::slice::Iter<'_, Made> as Iterator>::enumerate` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<core::slice::Iter<'_, Made> as Iterator>::filter` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<core::slice::Iter<'_, Made> as Iterator>::next` in `microkernel::cspace::Kernel::cs_retype_core`
callee `<objects::RetypeKind as PartialEq>::eq` in `microkernel::cspace::Kernel::cs_retype_core`
callee `Result::map_err cspace::SlotRetypeError {cspace::SlotRetypeError::Cs}>` in `microkernel::cspace::Kernel::cs_retype_core`
callee `Result::map_err cspace::SlotRetypeError {cspace::SlotRetypeError::Obj}>` in `microkernel::cspace::Kernel::cs_retype_core`
callee `Rights::contains` in `microkernel::cspace::Kernel::cs_retype_core`
callee `alloc::vec::Vec::push` in `microkernel::cspace::Kernel::cs_retype_core`
loop `microkernel::cspace::Kernel::cs_retype_core` header bb76
loop `microkernel::cspace::Kernel::cs_retype_core` header bb111
loop `microkernel::cspace::Kernel::cs_retype_core` header bb120
loop `microkernel::cspace::Kernel::cs_retype_core` header bb129
loop `microkernel::cspace::Kernel::cs_retype_core` header bb165
loop `microkernel::cspace::Kernel::cs_retype_core` header bb177
loop `microkernel::cspace::Kernel::cs_retype_core` header bb191
callee `<Result<(), cspace::SlotRetypeError> as FromResidual<Result<Infallible, cspace::SlotRetypeError>>>::from_residual` in `microkernel::cspace::Kernel::cs_retype_ept`
callee `<Result<(synthyrion_caps::ObjKind, u64, synthyrion_caps::Rights, u64), cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_retype_ept`
callee `Result::map_err cspace::SlotRetypeError {cspace::SlotRetypeError::Cs}>` in `microkernel::cspace::Kernel::cs_retype_ept`
callee `<Result<(synthyrion_caps::ObjKind, u64, synthyrion_caps::Rights, u64), cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_sched_context`
callee `<Result<u64, cspace::SlotRetypeError> as FromResidual<Result<Infallible, cspace::SlotRetypeError>>>::from_residual` in `microkernel::cspace::Kernel::cs_sched_context`
callee `Result::map_err cspace::SlotRetypeError {cspace::SlotRetypeError::Cs}>` in `microkernel::cspace::Kernel::cs_sched_context`
callee `Rights::contains` in `microkernel::cspace::Kernel::cs_sched_context`
callee `<Result<u64, cspace::SlotRetypeError> as FromResidual<Result<Infallible, cspace::SlotRetypeError>>>::from_residual` in `microkernel::cspace::Kernel::cs_sched_context_consumed`
callee `<Result<u64, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_sched_context_consumed`
callee `Result::map_err cspace::SlotRetypeError {cspace::SlotRetypeError::Obj}>` in `microkernel::cspace::Kernel::cs_sched_context_consumed`
callee `<Result<synthyrion_abi::SysResult, cspace::SlotRetypeError> as FromResidual<Result<Infallible, cspace::SlotRetypeError>>>::from_residual` in `microkernel::cspace::Kernel::cs_sched_context_yield_to`
callee `<Result<u64, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_sched_context_yield_to`
callee `<Result<(), cspace::SlotRetypeError> as FromResidual<Result<Infallible, cspace::SlotRetypeError>>>::from_residual` in `microkernel::cspace::Kernel::cs_unbind_sched_context`
callee `<Result<u64, cspace::SlotRetypeError> as Try>::branch` in `microkernel::cspace::Kernel::cs_unbind_sched_context`
callee `Result::map_err cspace::SlotRetypeError {cspace::SlotRetypeError::Obj}>` in `microkernel::cspace::Kernel::cs_unbind_sched_context`
callee `<&alloc::vec::Vec<(u64, CapNode)> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::cspace_invariants`
callee `<&alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::cspace_invariants`
callee `<(synthyrion_caps::ObjKind, u64) as PartialEq>::ne` in `microkernel::cspace::Kernel::cspace_invariants`
callee `<(u64, usize) as PartialEq>::ne` in `microkernel::cspace::Kernel::cspace_invariants`
callee `<Result<(), String> as FromResidual<Result<Infallible, String>>>::from_residual` in `microkernel::cspace::Kernel::cspace_invariants`
callee `<Result<(), String> as Try>::branch` in `microkernel::cspace::Kernel::cspace_invariants`
callee `<Result<Option<u64>, cspace::CsError> as PartialEq>::ne` in `microkernel::cspace::Kernel::cspace_invariants`
callee `<core::slice::Iter<'_, (u64, CapNode)> as Iterator>::next` in `microkernel::cspace::Kernel::cspace_invariants`
callee `Arguments::new` in `microkernel::cspace::Kernel::cspace_invariants`
callee `BTreeSet::insert` in `microkernel::cspace::Kernel::cspace_invariants`
callee `Rights::contains` in `microkernel::cspace::Kernel::cspace_invariants`
callee `alloc::collections::BTreeMap::entry` in `microkernel::cspace::Kernel::cspace_invariants`
callee `alloc::collections::btree_map::Entry::or_default` in `microkernel::cspace::Kernel::cspace_invariants`
callee `alloc::fmt::format` in `microkernel::cspace::Kernel::cspace_invariants`
callee `alloc::vec::Vec::push` in `microkernel::cspace::Kernel::cspace_invariants`
callee `core::fmt::rt::Argument::new_display` in `microkernel::cspace::Kernel::cspace_invariants`
callee `core::fmt::rt::Argument::new_lower_hex` in `microkernel::cspace::Kernel::cspace_invariants`
callee `core::slice::contains` in `microkernel::cspace::Kernel::cspace_invariants`
callee `must_use` in `microkernel::cspace::Kernel::cspace_invariants`
loop `microkernel::cspace::Kernel::cspace_invariants` header bb2
loop `microkernel::cspace::Kernel::cspace_invariants` header bb43
loop `microkernel::cspace::Kernel::cspace_invariants` header bb151
callee `<TreeRo<'_> as LinkAccess>::links` in `microkernel::cspace::Kernel::descendants_walk`
loop `microkernel::cspace::Kernel::descendants_walk` header bb3
loop `microkernel::cspace::Kernel::drain` header bb1
loop `microkernel::cspace::Kernel::drain_cn_gone` header bb2
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::cspace::Kernel::drop_born_nodes`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::drop_born_nodes`
loop `microkernel::cspace::Kernel::drop_born_nodes` header bb2
loop `microkernel::cspace::Kernel::drop_born_nodes` header bb17
callee `<Enumerate<core::slice::Iter<'_, XferCap>> as IntoIterator>::into_iter` in `microkernel::cspace::Kernel::install_xfer_journaled`
callee `<Enumerate<core::slice::Iter<'_, XferCap>> as Iterator>::next` in `microkernel::cspace::Kernel::install_xfer_journaled`
callee `<core::slice::Iter<'_, XferCap> as Iterator>::enumerate` in `microkernel::cspace::Kernel::install_xfer_journaled`
callee `Rights::contains` in `microkernel::cspace::Kernel::install_xfer_journaled`
loop `microkernel::cspace::Kernel::install_xfer_journaled` header bb25
callee `alloc::vec::Vec::push` in `microkernel::cspace::Kernel::kids_of`
loop `microkernel::cspace::Kernel::kids_of` header bb5
loop `microkernel::cspace::Kernel::next_occupied_slot` header bb8
callee `<Option<Option<u64>> as Try>::branch` in `microkernel::cspace::Kernel::resolve`
callee `<Result<XferList, cspace::CsError> as FromResidual<Result<Infallible, cspace::CsError>>>::from_residual` in `microkernel::cspace::Kernel::resolve_xfer`
loop `microkernel::cspace::Kernel::resolve_xfer` header bb6
loop `microkernel::cspace::Kernel::revoke_ids` header bb16
callee `Rights::contains` in `microkernel::cspace::Kernel::send_ids`
loop `microkernel::cspace::Kernel::uninstall_xfer` header bb1
callee `<FilterMap<core::slice::Iter<'_, u64>, &{closure@crates\microkernel\src\cspace.rs:886:18: 886:27}> as Iterator>::collect` in `microkernel::cspace::Kernel::view`
callee `<core::slice::Iter<'_, u64> as Iterator>::filter_map` in `microkernel::cspace::Kernel::view`
callee `<KernelC as Default>::default` in `microkernel::cspace::KernelC::new`
callee `<CnFree as Clone>::clone` in `microkernel::cspace::clone`
callee `<Deferred as Clone>::clone` in `microkernel::cspace::clone`
callee `<FrameNodes as Clone>::clone` in `microkernel::cspace::clone`
callee `<NodeIds as Clone>::clone` in `microkernel::cspace::clone`
callee `<Option<(u64, usize)> as Clone>::clone` in `microkernel::cspace::clone`
callee `<Option<Job> as Clone>::clone` in `microkernel::cspace::clone`
callee `<PendingStack as Clone>::clone` in `microkernel::cspace::clone`
callee `<synthyrion_caps::ObjKind as Clone>::clone` in `microkernel::cspace::clone`
callee `<synthyrion_caps::Rights as Clone>::clone` in `microkernel::cspace::clone`
callee `<alloc::vec::Vec<(u64, usize)> as Clone>::clone` in `microkernel::cspace::clone`
callee `<alloc::vec::Vec<u64> as Clone>::clone` in `microkernel::cspace::clone`
callee `<Deferred as Default>::default` in `microkernel::cspace::default`
callee `<NodeIds as Default>::default` in `microkernel::cspace::default`
callee `<PendingStack as Default>::default` in `microkernel::cspace::default`
callee `<u64 as Default>::default` in `microkernel::cspace::default`
callee `<u8 as Default>::default` in `microkernel::cspace::default`
callee `alloc::vec::from_elem` in `microkernel::cspace::default`
callee `<&cspace::CsError as PartialEq>::eq` in `microkernel::cspace::eq`
callee `<&cspace::LookupFault as PartialEq>::eq` in `microkernel::cspace::eq`
callee `<&objects::ObjError as PartialEq>::eq` in `microkernel::cspace::eq`
callee `<Option<(u64, usize)> as PartialEq>::eq` in `microkernel::cspace::eq`
callee `<synthyrion_caps::Rights as PartialEq>::eq` in `microkernel::cspace::eq`
callee `<alloc::vec::Vec<(u64, usize)> as PartialEq>::eq` in `microkernel::cspace::eq`
callee `Formatter::debug_struct_field2_finish` in `microkernel::cspace::fmt`
callee `Formatter::debug_struct_field3_finish` in `microkernel::cspace::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::cspace::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::cspace::fmt`
callee `Formatter::write_str` in `microkernel::cspace::fmt`
callee `<u64 as Ord>::clamp` in `microkernel::cspace::nodeid::NodeIds::purge_restart_at`
callee `<u64 as Ord>::cmp` in `microkernel::cspace::nodeid::cmp`
callee `Formatter::debug_struct_fields_finish` in `microkernel::cspace::nodeid::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::cspace::nodeid::fmt`
callee `<u64 as Hash>::hash` in `microkernel::cspace::nodeid::hash`
callee `<u64 as PartialOrd>::partial_cmp` in `microkernel::cspace::nodeid::partial_cmp`
loop `microkernel::epoch_kind_sum` header bb1
callee `<ManuallyDrop<ObjRam> as Deref>::deref` in `microkernel::epochs::EpochTable::attach_window`
callee `ManuallyDrop::into_inner` in `microkernel::epochs::EpochTable::attach_window`
callee `ManuallyDrop::new` in `microkernel::epochs::EpochTable::attach_window`
callee `<dyn userwalk::PhysMem as userwalk::PhysMem>::sub_window` in `microkernel::epochs::KernelAuthority::attach_epoch_window_from`
callee `CapAuthority::new512` in `microkernel::epochs::KernelAuthority::new`
callee `RefCell::new` in `microkernel::epochs::KernelAuthority::new`
callee `Formatter::write_str` in `microkernel::epochs::fmt`
loop `microkernel::ept_tables::Kernel::ept_holds_write_execute` header bb8
loop `microkernel::ept_tables::Kernel::ept_leaf_count` header bb2
callee `<Option<Option<u8>> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::ept_tables::Kernel::ept_missing_level`
callee `<Result<&synthyrion_capauth::SealedCap, vcpu::HvError> as Try>::branch` in `microkernel::ept_tables::Kernel::ept_provision`
callee `<Result<alloc::vec::Vec<synthyrion_capauth::SealedCap>, vcpu::HvError> as Try>::branch` in `microkernel::ept_tables::Kernel::ept_provision`
callee `<alloc::vec::Vec<synthyrion_capauth::SealedCap> as Deref>::deref` in `microkernel::ept_tables::Kernel::ept_provision`
loop `microkernel::ept_tables::Kernel::ept_provision` header bb2
callee `<objects::ObjError as PartialEq>::eq` in `microkernel::ept_tables::Kernel::ept_provision::{closure#0}`
callee `alloc::collections::BTreeMap::insert` in `microkernel::ept_tables::Kernel::ept_provision_one`
callee `eptp` in `microkernel::ept_tables::Kernel::ept_table_pointer`
callee `hypervisor::Perms::allows` in `microkernel::ept_tables::Kernel::ept_walk_access`
callee `<Filter<alloc::collections::btree_map::Iter<'_, u64, TableRec>, {closure@crates\microkernel\src\vcpu\ept_tables.rs:742:63: 742:71}> as Iterator>::map` in `microkernel::ept_tables::Kernel::hv_destroy_table`
callee `<alloc::collections::btree_map::Iter<'_, u64, TableRec> as Iterator>::filter` in `microkernel::ept_tables::Kernel::hv_destroy_table`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::ept_tables::Kernel::hv_destroy_table`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::ept_tables::Kernel::hv_destroy_table`
callee `<core::iter::Map<Filter<alloc::collections::btree_map::Iter<'_, u64, TableRec>, {closure@crates\microkernel\src\vcpu\ept_tables.rs:742:63: 742:71}>, {closure@crates\microkernel\src\vcpu\ept_tables.rs:742:213: 742:221}> as Iterator>::collect` in `microkernel::ept_tables::Kernel::hv_destroy_table`
callee `and_then (2 closure position(s) seen but 3 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::ept_tables::Kernel::hv_destroy_table`
loop `microkernel::ept_tables::Kernel::hv_destroy_table` header bb22
callee `<Owner as PartialEq>::eq` in `microkernel::ept_tables::Kernel::hv_destroy_table::{closure#1}::{closure#0}`
loop `microkernel::ept_tables::Kernel::hv_mirror_span` header bb3
callee `alloc::collections::BTreeMap::insert` in `microkernel::ept_tables::Kernel::hv_new_table`
callee `<Filter<alloc::collections::btree_map::Iter<'_, u64, TableRec>, {closure@crates\microkernel\src\vcpu\ept_tables.rs:771:58: 771:66}> as Iterator>::map` in `microkernel::ept_tables::Kernel::hv_release_tables`
callee `<alloc::collections::btree_map::Iter<'_, u64, TableRec> as Iterator>::filter` in `microkernel::ept_tables::Kernel::hv_release_tables`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::ept_tables::Kernel::hv_release_tables`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::ept_tables::Kernel::hv_release_tables`
callee `<core::iter::Map<Filter<alloc::collections::btree_map::Iter<'_, u64, TableRec>, {closure@crates\microkernel\src\vcpu\ept_tables.rs:771:58: 771:66}>, {closure@crates\microkernel\src\vcpu\ept_tables.rs:771:121: 771:129}> as Iterator>::collect` in `microkernel::ept_tables::Kernel::hv_release_tables`
loop `microkernel::ept_tables::Kernel::hv_release_tables` header bb6
callee `<Owner as PartialEq>::eq` in `microkernel::ept_tables::Kernel::hv_release_tables::{closure#0}::{closure#0}`
loop `microkernel::ept_tables::Kernel::tree_leaf_from` header bb7
callee `<&u64 as PartialEq>::eq` in `microkernel::ept_tables::eq`
callee `Formatter::write_str` in `microkernel::filedoor::fmt`
callee `alloc::vec::Vec::resize` in `microkernel::framelink::FrameLinks::new`
callee `alloc::collections::BTreeMap::contains_key` in `microkernel::framelink::FrameLinks::pool_push`
callee `alloc::collections::BTreeMap::insert` in `microkernel::framelink::FrameLinks::pool_push`
callee `Formatter::debug_struct_field2_finish` in `microkernel::framelink::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::framelink::fmt`
callee `Formatter::debug_struct_field2_finish` in `microkernel::framemap::fmt`
callee `Formatter::write_str` in `microkernel::framemap::fmt`
callee `Cell::new` in `microkernel::framenodes::FrameNodes::new`
callee `alloc::vec::from_elem` in `microkernel::framenodes::FrameNodes::new`
loop `microkernel::framenodes::FrameNodes::register_cnode` header bb17
loop `microkernel::framenodes::FrameNodes::register_cnode` header bb35
callee `<Cell<bool> as Clone>::clone` in `microkernel::framenodes::clone`
callee `<Cell<u64> as Clone>::clone` in `microkernel::framenodes::clone`
callee `<alloc::vec::Vec<u64> as Clone>::clone` in `microkernel::framenodes::clone`
callee `<synthyrion_caps::Rights as PartialEq>::eq` in `microkernel::framenodes::eq`
callee `Formatter::debug_struct_fields_finish` in `microkernel::framenodes::fmt`
callee `alloc::vec::Vec::resize` in `microkernel::frametab::FrameTab::new`
callee `<Enumerate<core::slice::Iter<'_, [u64; 5]>> as Iterator>::scan` in `microkernel::frametab::FrameTab::referenced`
callee `<core::iter::Scan<Enumerate<core::slice::Iter<'_, [u64; 5]>>, usize, {closure@crates\microkernel\src\frametab.rs:371:30: 371:44}> as Iterator>::flatten` in `microkernel::frametab::FrameTab::referenced`
callee `<core::slice::Iter<'_, [u64; 5]> as Iterator>::enumerate` in `microkernel::frametab::FrameTab::referenced`
loop `microkernel::gate::CptrList::from_bytes` header bb5
callee `core::num::from_le_bytes` in `microkernel::gate::CptrList::word`
callee `<[u8; 8] as TryFrom<&[u8]>>::try_from` in `microkernel::gate::CptrList::word::{closure#0}`
callee `Rights::contains` in `microkernel::gate::Kernel::gate_receive`
callee `Rights::contains` in `microkernel::gate::Kernel::serve`
callee `Rights::contains` in `microkernel::gate::Kernel::serve::{closure#0}`
callee `<&ioport::PortAccess as PartialEq>::eq` in `microkernel::gate::eq`
callee `<&u64 as PartialEq>::eq` in `microkernel::gate::eq`
callee `<[u64; 4] as PartialEq>::eq` in `microkernel::gate::eq`
callee `<gate::After as PartialEq>::eq` in `microkernel::gate::eq`
callee `Formatter::debug_struct_field3_finish` in `microkernel::gate::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::gate::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::gate::fmt`
callee `Formatter::write_str` in `microkernel::gate::fmt`
callee `<Option<synthyrion_caps::Rights> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::gate::rights_mask`
callee `Option::get_or_insert_with Box<DriverReports> {<Box<DriverReports> as Default>::default}>` in `microkernel::gate_driver::Kernel::driver_report_set`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::gate_driver::Kernel::gate_driver`
callee `Option::get_or_insert_with Box<DriverReports> {<Box<DriverReports> as Default>::default}>` in `microkernel::gate_driver::Kernel::gate_driver`
callee `Rights::contains` in `microkernel::gate_driver::Kernel::gate_driver`
callee `core::ops::RangeInclusive::contains` in `microkernel::gate_driver::Kernel::gate_driver`
callee `<[u64; 16] as Default>::default` in `microkernel::gate_driver::default`
callee `<u16 as Default>::default` in `microkernel::gate_driver::default`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::gate_taskcfg::Kernel::gate_taskcfg`
callee `Rights::contains` in `microkernel::gate_taskcfg::Kernel::gate_taskcfg`
callee `core::ops::RangeInclusive::contains` in `microkernel::gate_taskcfg::Kernel::gate_taskcfg`
callee `Rights::contains` in `microkernel::gate_taskcfg::Kernel::gate_taskcfg::{closure#0}`
callee `Rights::contains` in `microkernel::gate_taskcfg::Kernel::gate_taskcfg::{closure#1}`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::gate_virt::Kernel::gate_virt`
callee `Option::map_or u64 {<u64 as From<u8>>::from}>` in `microkernel::gate_virt::Kernel::gate_virt`
loop `microkernel::gate_virt::Kernel::gate_virt` header bb55
loop `microkernel::gate_virt::Kernel::gate_virt` header bb66
loop `microkernel::gate_virt::Kernel::gate_virt` header bb94
loop `microkernel::gate_virt::Kernel::gate_virt` header bb182
loop `microkernel::gate_virt::Kernel::gate_virt` header bb193
loop `microkernel::gate_virt::Kernel::gate_virt` header bb215
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::gate_virt::Kernel::virt_cap`
callee `Rights::contains` in `microkernel::gate_virt::Kernel::virt_cap`
callee `Rights::contains` in `microkernel::gate_vspace::Kernel::exec_auth_at`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::gate_vspace::Kernel::gate_vspace`
callee `Rights::contains` in `microkernel::gate_vspace::Kernel::gate_vspace`
callee `core::ops::RangeInclusive::contains` in `microkernel::gate_vspace::Kernel::gate_vspace`
callee `Rights::contains` in `microkernel::gate_vspace::Kernel::gate_vspace::{closure#0}`
callee `Rights::contains` in `microkernel::gate_vspace::Kernel::table_frame_at`
loop `microkernel::guardmap::Dual::agrees` header bb5
callee `<core::iter::Map<core::ops::Range<usize>, {closure@crates\microkernel\src\guardmap.rs:639:44: 639:47}> as Iterator>::collect` in `microkernel::guardmap::Dual::heal`
callee `<core::ops::Range<usize> as Iterator>::all` in `microkernel::guardmap::Dual::heal`
callee `<core::ops::Range<usize> as Iterator>::any` in `microkernel::guardmap::Dual::heal`
callee `<core::ops::Range<usize> as Iterator>::map` in `microkernel::guardmap::Dual::heal`
callee `GuardedVec::parity_ok` in `microkernel::guardmap::Dual::heal`
callee `GuardedVec::scrub_all` in `microkernel::guardmap::Dual::heal`
callee `GuardedVec::word_ok` in `microkernel::guardmap::Dual::heal::{closure#0}`
callee `GuardedVec::word_ok` in `microkernel::guardmap::Dual::heal::{closure#2}`
callee `<alloc::vec::Vec<u64> as Clone>::clone` in `microkernel::guardmap::Dual::new`
callee `GuardedVec::from_words` in `microkernel::guardmap::Dual::new`
callee `<alloc::vec::Vec<u64> as Clone>::clone` in `microkernel::guardmap::Dual::reseal`
callee `GuardedVec::from_words` in `microkernel::guardmap::Dual::reseal`
callee `<T as Rec>::set_children` in `microkernel::guardmap::Entry::heal_from_mirror`
callee `<core::ops::Range<usize> as Iterator>::all` in `microkernel::guardmap::Entry::heal_from_mirror`
callee `GuardedVec::reload` in `microkernel::guardmap::Entry::heal_from_mirror`
callee `GuardedVec::word_ok` in `microkernel::guardmap::Entry::heal_from_mirror::{closure#0}`
callee `GuardedVec::from_words` in `microkernel::guardmap::Entry::new`
callee `alloc::slice::to_vec` in `microkernel::guardmap::Entry::new`
callee `<GMut<'_, T> as DerefMut>::deref_mut` in `microkernel::guardmap::GMap::for_each_mut`
callee `<alloc::collections::btree_map::IterMut<'_, u64, guardmap::Entry<T>> as IntoIterator>::into_iter` in `microkernel::guardmap::GMap::for_each_mut`
callee `<alloc::collections::btree_map::IterMut<'_, u64, guardmap::Entry<T>> as Iterator>::next` in `microkernel::guardmap::GMap::for_each_mut`
callee `<impl FnMut(&mut T) as FnMut<(&mut T,)>>::call_mut` in `microkernel::guardmap::GMap::for_each_mut`
loop `microkernel::guardmap::GMap::for_each_mut` header bb3
callee `core::slice::sort_unstable` in `microkernel::guardmap::GMap::heal_children_from_mirror`
callee `alloc::collections::BTreeMap::insert` in `microkernel::guardmap::GMap::insert`
callee `<Filter<alloc::collections::btree_map::Iter<'_, u64, guardmap::Entry<T>>, {closure@crates\microkernel\src\guardmap.rs:451:21: 451:29}> as Iterator>::map` in `microkernel::guardmap::GMap::members_where`
callee `<alloc::collections::btree_map::Iter<'_, u64, guardmap::Entry<T>> as Iterator>::filter` in `microkernel::guardmap::GMap::members_where`
callee `<core::iter::Map<Filter<alloc::collections::btree_map::Iter<'_, u64, guardmap::Entry<T>>, {closure@crates\microkernel\src\guardmap.rs:451:21: 451:29}>, {closure@crates\microkernel\src\guardmap.rs:452:18: 452:27}> as Iterator>::collect` in `microkernel::guardmap::GMap::members_where`
callee `<alloc::collections::btree_map::Range<'_, u64, guardmap::Entry<T>> as Iterator>::next` in `microkernel::guardmap::GMap::next_id`
callee `<Filter<alloc::collections::btree_map::Iter<'_, u64, guardmap::Entry<T>>, {closure@crates\microkernel\src\guardmap.rs:458:52: 458:60}> as Iterator>::map` in `microkernel::guardmap::GMap::parents_of_any`
callee `<alloc::collections::btree_map::Iter<'_, u64, guardmap::Entry<T>> as Iterator>::filter` in `microkernel::guardmap::GMap::parents_of_any`
callee `<core::iter::Map<Filter<alloc::collections::btree_map::Iter<'_, u64, guardmap::Entry<T>>, {closure@crates\microkernel\src\guardmap.rs:458:52: 458:60}>, {closure@crates\microkernel\src\guardmap.rs:458:115: 458:124}> as Iterator>::collect` in `microkernel::guardmap::GMap::parents_of_any`
callee `alloc::vec::Vec::push` in `microkernel::guardmap::GMap::parents_of_any`
callee `core::slice::contains` in `microkernel::guardmap::GMap::parents_of_any`
callee `filter (2 closure position(s) seen but 3 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::guardmap::GMap::parents_of_any`
callee `core::slice::contains` in `microkernel::guardmap::GMap::parents_of_any::{closure#0}`
callee `<Cell<bool> as Clone>::clone` in `microkernel::guardmap::clone`
callee `<Cell<u64> as Clone>::clone` in `microkernel::guardmap::clone`
callee `<Guarded<10, u64::MAX> as Clone>::clone` in `microkernel::guardmap::clone`
callee `<GuardedVec<u64::MAX> as Clone>::clone` in `microkernel::guardmap::clone`
callee `<T as Clone>::clone` in `microkernel::guardmap::clone`
callee `<alloc::collections::BTreeMap<u64, guardmap::Entry<T>> as Clone>::clone` in `microkernel::guardmap::clone`
callee `<alloc::vec::Vec<u64> as Clone>::clone` in `microkernel::guardmap::clone`
callee `Cell::new` in `microkernel::guardmap::default`
callee `Formatter::debug_struct_field3_finish` in `microkernel::guardmap::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::guardmap::fmt`
callee `Formatter::write_str` in `microkernel::guardmap::fmt`
loop `microkernel::guardmap::fold` header bb1
callee `<alloc::collections::btree_map::Iter<'_, u64, guardmap::Entry<T>> as Iterator>::map (&u64, &T)>` in `microkernel::guardmap::into_iter`
callee `GuardedVec::parity_ok` in `microkernel::guardmap::parity_ok`
callee `GuardedVec::raw_words` in `microkernel::guardmap::raw_words`
callee `GuardedArr::reload_as` in `microkernel::guardmap::reseal_as`
callee `GuardedVec::reload_as` in `microkernel::guardmap::reseal_as`
callee `GuardedArr::scrub_all` in `microkernel::guardmap::scrub_all`
callee `GuardedVec::scrub_all` in `microkernel::guardmap::scrub_all`
callee `GuardedVec::word_ok` in `microkernel::guardmap::word_ok`
callee `<Enumerate<core::slice::Iter<'_, (u64, u64, bool)>> as IntoIterator>::into_iter` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<Enumerate<core::slice::Iter<'_, (u64, u64, bool)>> as Iterator>::next` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<Option<usize> as PartialEq>::ne` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<Result<(), GuestIoError> as FromResidual<Result<Infallible, GuestIoError>>>::from_residual` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<Result<u64, GuestIoError> as Try>::branch` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<Take<core::slice::Iter<'_, (u64, u64, bool)>> as IntoIterator>::into_iter` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<Take<core::slice::Iter<'_, (u64, u64, bool)>> as Iterator>::next` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<alloc::vec::Vec<(u64, u64, bool)> as Deref>::deref` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<core::slice::Iter<'_, (u64, u64, bool)> as IntoIterator>::into_iter` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<core::slice::Iter<'_, (u64, u64, bool)> as Iterator>::enumerate` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<core::slice::Iter<'_, (u64, u64, bool)> as Iterator>::next` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `<core::slice::Iter<'_, (u64, u64, bool)> as Iterator>::take` in `microkernel::guestio::Kernel::ept_bind_iospace`
callee `alloc::vec::Vec::push` in `microkernel::guestio::Kernel::ept_bind_iospace`
loop `microkernel::guestio::Kernel::ept_bind_iospace` header bb30
loop `microkernel::guestio::Kernel::ept_bind_iospace` header bb43
loop `microkernel::guestio::Kernel::ept_bind_iospace` header bb53
loop `microkernel::guestio::Kernel::ept_bind_iospace` header bb63
callee `<iospace::IoError as PartialEq>::eq` in `microkernel::guestio::Kernel::ept_bind_iospace::{closure#1}`
callee `<Result<u64, DmaFault> as FromResidual<Result<Infallible, DmaFault>>>::from_residual` in `microkernel::guestio::Kernel::guest_dma_check`
callee `<Result<u64, DmaFault> as Try>::branch` in `microkernel::guestio::Kernel::guest_dma_check`
callee `<alloc::vec::IntoIter<(u64, u64, bool)> as Iterator>::next` in `microkernel::guestio::Kernel::guest_io_invariant`
callee `<alloc::vec::Vec<(u64, u64, bool)> as IntoIterator>::into_iter` in `microkernel::guestio::Kernel::guest_io_invariant`
loop `microkernel::guestio::Kernel::guest_io_invariant` header bb12
loop `microkernel::guestio::Kernel::hv_mirror_release` header bb4
callee `<&iospace::IoError as PartialEq>::eq` in `microkernel::guestio::eq`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::guestio::fmt`
callee `Formatter::write_str` in `microkernel::guestio::fmt`
callee `core::num::div_ceil` in `microkernel::hdr_frames_for`
callee `<C as KidGuard>::len` in `microkernel::heal_parts`
callee `<C as KidGuard>::parity_ok` in `microkernel::heal_parts`
callee `<C as KidGuard>::raw_words` in `microkernel::heal_parts`
callee `<C as KidGuard>::reseal_as` in `microkernel::heal_parts`
callee `<C as KidGuard>::scrub_all` in `microkernel::heal_parts`
callee `<Enumerate<core::slice::Iter<'_, u64>> as Iterator>::take` in `microkernel::heal_parts`
callee `<RecHeal as PartialEq>::eq` in `microkernel::heal_parts`
callee `<T as Rec>::set_children` in `microkernel::heal_parts`
callee `<T as Rec>::set_scalars` in `microkernel::heal_parts`
callee `<Take<Enumerate<core::slice::Iter<'_, u64>>> as IntoIterator>::into_iter` in `microkernel::heal_parts`
callee `<Take<Enumerate<core::slice::Iter<'_, u64>>> as Iterator>::next` in `microkernel::heal_parts`
callee `<core::ops::Range<usize> as Iterator>::all` in `microkernel::heal_parts`
callee `<core::ops::Range<usize> as Iterator>::any` in `microkernel::heal_parts`
callee `core::array::from_fn` in `microkernel::heal_parts`
loop `microkernel::heal_parts` header bb33
callee `<C as KidGuard>::word_ok` in `microkernel::heal_parts::{closure#1}`
callee `<C as KidGuard>::raw` in `microkernel::heal_parts::{closure#4}`
callee `<C as KidGuard>::word_ok` in `microkernel::heal_parts::{closure#4}`
callee `<Push as PartialEq>::ne` in `microkernel::inboxring::InboxRing::retain`
callee `<dyn for<'a> FnMut(&'a Delivery) -> bool as FnMut<(&Delivery,)>>::call_mut` in `microkernel::inboxring::InboxRing::retain`
loop `microkernel::inboxring::InboxRing::retain` header bb5
callee `alloc::vec::Vec::push` in `microkernel::inboxring::InboxRing::to_vec`
loop `microkernel::inboxring::InboxRing::to_vec` header bb6
callee `<&Delivery as PartialEq>::eq` in `microkernel::inboxring::eq`
callee `Formatter::debug_struct_field1_finish` in `microkernel::inboxring::fmt`
callee `Formatter::debug_struct_field5_finish` in `microkernel::inboxring::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::inboxring::fmt`
callee `Formatter::write_str` in `microkernel::inboxring::fmt`
callee `<&[(usize, usize)] as IntoIterator>::into_iter` in `microkernel::inherited_priorities`
callee `<Option<u8> as PartialOrd>::gt` in `microkernel::inherited_priorities`
callee `<alloc::vec::Vec<Option<u8>> as Deref>::deref` in `microkernel::inherited_priorities`
callee `<alloc::vec::Vec<Option<u8>> as DerefMut>::deref_mut` in `microkernel::inherited_priorities`
callee `<core::slice::Iter<'_, (usize, usize)> as Iterator>::next` in `microkernel::inherited_priorities`
callee `alloc::slice::to_vec` in `microkernel::inherited_priorities`
loop `microkernel::inherited_priorities` header bb3
callee `<u16 as TryFrom<u64>>::try_from` in `microkernel::io_rows::Kernel::serve_io_rows`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::io_rows::Kernel::serve_io_rows`
callee `core::ops::RangeInclusive::contains` in `microkernel::io_tree::Kernel::iot_install`
callee `alloc::collections::BTreeMap::insert` in `microkernel::io_tree::Kernel::iot_provision_one`
callee `<alloc::collections::btree_map::Iter<'_, u64, TableRec> as Iterator>::find` in `microkernel::io_tree::Kernel::iot_release_tables`
loop `microkernel::io_tree::Kernel::iot_release_tables` header bb1
callee `<Owner as PartialEq>::eq` in `microkernel::io_tree::Kernel::iot_release_tables::{closure#0}::{closure#0}`
callee `Rights::contains` in `microkernel::ioport::Kernel::cs_ioport`
callee `Rights::contains` in `microkernel::ioport::Kernel::cs_ioport_control_ok`
callee `<Result<(u16, u16), ioport::IoPortError> as Try>::branch` in `microkernel::ioport::Kernel::cs_ioport_in`
callee `<Result<ioport::PortAccess, ioport::IoPortError> as FromResidual<Result<Infallible, ioport::IoPortError>>>::from_residual` in `microkernel::ioport::Kernel::cs_ioport_in`
callee `<Result<(), ioport::IoPortError> as FromResidual<Result<Infallible, ioport::IoPortError>>>::from_residual` in `microkernel::ioport::Kernel::cs_ioport_issue`
callee `<Result<(), ioport::IoPortError> as Try>::branch` in `microkernel::ioport::Kernel::cs_ioport_issue`
callee `<Result<Plan, ioport::IoPortError> as Try>::branch` in `microkernel::ioport::Kernel::cs_ioport_issue`
callee `<Result<(u16, u16), ioport::IoPortError> as Try>::branch` in `microkernel::ioport::Kernel::cs_ioport_out`
callee `<Result<ioport::PortAccess, ioport::IoPortError> as FromResidual<Result<Infallible, ioport::IoPortError>>>::from_residual` in `microkernel::ioport::Kernel::cs_ioport_out`
callee `<Result<(), ioport::IoPortError> as FromResidual<Result<Infallible, ioport::IoPortError>>>::from_residual` in `microkernel::ioport::Kernel::cs_ioport_revoke`
callee `<Result<(), ioport::IoPortError> as Try>::branch` in `microkernel::ioport::Kernel::cs_ioport_revoke`
callee `<Result<(u16, u16), ioport::IoPortError> as Try>::branch` in `microkernel::ioport::Kernel::ioport_in`
callee `<Result<ioport::PortAccess, ioport::IoPortError> as FromResidual<Result<Infallible, ioport::IoPortError>>>::from_residual` in `microkernel::ioport::Kernel::ioport_in`
callee `<Result<Plan, ioport::IoPortError> as Try>::branch` in `microkernel::ioport::Kernel::ioport_issue`
callee `<Result<synthyrion_capauth::SealedCap, ioport::IoPortError> as FromResidual<Result<Infallible, ioport::IoPortError>>>::from_residual` in `microkernel::ioport::Kernel::ioport_issue`
callee `<Filter<core::slice::Iter<'_, PortRange>, {closure@crates\microkernel\src\ioport.rs:335:44: 335:47}> as Iterator>::count` in `microkernel::ioport::Kernel::ioport_live_ranges`
callee `<core::slice::Iter<'_, PortRange> as Iterator>::filter` in `microkernel::ioport::Kernel::ioport_live_ranges`
callee `<Result<(u16, u16), ioport::IoPortError> as Try>::branch` in `microkernel::ioport::Kernel::ioport_out`
callee `<Result<ioport::PortAccess, ioport::IoPortError> as FromResidual<Result<Infallible, ioport::IoPortError>>>::from_residual` in `microkernel::ioport::Kernel::ioport_out`
callee `<core::slice::IterMut<'_, PortRange> as Iterator>::find` in `microkernel::ioport::Kernel::ioport_revoke_port`
callee `<u16 as TryFrom<u64>>::try_from` in `microkernel::ioport::Kernel::syscall_ioport`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::ioport::Kernel::syscall_ioport`
callee `<core::slice::Iter<'_, PortRange> as Iterator>::any` in `microkernel::ioport::KernelP::plan`
callee `<core::slice::Iter<'_, PortRange> as Iterator>::position` in `microkernel::ioport::KernelP::plan`
callee `Rights::contains` in `microkernel::ioport::KernelP::plan`
callee `<[PortRange; 64] as Clone>::clone` in `microkernel::ioport::clone`
callee `<&cspace::CsError as PartialEq>::eq` in `microkernel::ioport::eq`
callee `<Option<synthyrion_capauth::SealedCap> as PartialEq>::eq` in `microkernel::ioport::eq`
callee `<Option<ioport::PortAccess> as PartialEq>::eq` in `microkernel::ioport::eq`
callee `<Option<u32> as PartialEq>::eq` in `microkernel::ioport::eq`
callee `Formatter::debug_struct_field3_finish` in `microkernel::ioport::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::ioport::fmt`
callee `Formatter::write_str` in `microkernel::ioport::fmt`
callee `Box::new` in `microkernel::iospace::Kernel::attach_iommu`
callee `Box::new` in `microkernel::iospace::Kernel::cs_attach_iommu`
callee `core::mem::swap` in `microkernel::iospace::Kernel::cs_attach_iommu`
callee `Rights::contains` in `microkernel::iospace::Kernel::cs_io_space_id`
callee `Rights::contains` in `microkernel::iospace::Kernel::cs_iommu_authority`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::cs_iospace_attach_device`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::cs_iospace_create`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::cs_iospace_destroy`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::cs_iospace_invalidate`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::cs_iospace_map`
callee `Rights::contains` in `microkernel::iospace::Kernel::cs_iospace_map`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::cs_iospace_unmap`
callee `<Option<(usize, u32)> as Try>::branch` in `microkernel::iospace::Kernel::guest_io_count`
callee `<Option<(usize, u32)> as Try>::branch` in `microkernel::iospace::Kernel::guest_io_lookup`
callee `<Option<IoLeaf> as Try>::branch` in `microkernel::iospace::Kernel::guest_io_lookup`
callee `alloc::vec::Vec::push` in `microkernel::iospace::Kernel::guest_io_pages::{closure#0}`
callee `<dyn iospace::IoMmu as iospace::IoMmu>::attach_device` in `microkernel::iospace::Kernel::io_attach_core`
callee `<dyn iospace::IoMmu as iospace::IoMmu>::detach_device` in `microkernel::iospace::Kernel::io_attach_core`
callee `<dyn iospace::IoMmu as iospace::IoMmu>::destroy_space` in `microkernel::iospace::Kernel::io_destroy_core`
callee `<dyn iospace::IoMmu as iospace::IoMmu>::detach_device` in `microkernel::iospace::Kernel::io_destroy_core`
loop `microkernel::iospace::Kernel::io_destroy_core` header bb6
loop `microkernel::iospace::Kernel::io_destroy_core` header bb14
callee `<dyn iospace::IoMmu as iospace::IoMmu>::invalidate` in `microkernel::iospace::Kernel::io_invalidate_core`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::iospace_attach_device`
callee `<Result<synthyrion_capauth::SealedCap, iospace::IoError> as FromResidual<Result<Infallible, iospace::IoError>>>::from_residual` in `microkernel::iospace::Kernel::iospace_create`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::iospace_create`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::iospace_destroy`
loop `microkernel::iospace::Kernel::iospace_drop_frame` header bb6
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::iospace_invalidate`
callee `<Option<IoLeaf> as Try>::branch` in `microkernel::iospace::Kernel::iospace_lookup`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::iospace_map`
callee `<Option<&IoHome> as Try>::branch` in `microkernel::iospace::Kernel::iospace_of_device`
callee `<Option<(usize, usize)> as Try>::branch` in `microkernel::iospace::Kernel::iospace_of_device`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::Kernel::iospace_unmap`
callee `<Result<&Made, objects::ObjError> as Try>::branch` in `microkernel::iospace::Kernel::retype_iospace`
callee `<Result<synthyrion_capauth::SealedCap, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::iospace::Kernel::retype_iospace`
callee `<Result<alloc::vec::Vec<Made>, objects::ObjError> as Try>::branch` in `microkernel::iospace::Kernel::retype_iospace`
callee `<alloc::vec::Vec<Made> as Deref>::deref` in `microkernel::iospace::Kernel::retype_iospace`
callee `<&cspace::CsError as PartialEq>::eq` in `microkernel::iospace::eq`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::iospace::fmt`
callee `Formatter::write_str` in `microkernel::iospace::fmt`
callee `<core::slice::Iter<'_, IoHome> as Iterator>::position` in `microkernel::iospace::record::IoIndex::free_space_slot`
callee `<Filter<core::slice::Iter<'_, IoHome>, {closure@crates\microkernel\src\iospace\record.rs:84:35: 84:38}> as Iterator>::count` in `microkernel::iospace::record::IoIndex::space_count`
callee `<core::slice::Iter<'_, IoHome> as Iterator>::filter` in `microkernel::iospace::record::IoIndex::space_count`
callee `<dyn iospace::IoMmu as iospace::IoMmu>::create_space` in `microkernel::iospace::record::Kernel::io_create_core_at`
loop `microkernel::iospace::record::Kernel::io_find_dev` header bb2
loop `microkernel::iospace::record::Kernel::io_first_dev_of_space` header bb5
callee `<impl FnMut(IoLeaf) as FnMut<(IoLeaf,)>>::call_mut` in `microkernel::iospace::record::Kernel::io_for_each_map`
loop `microkernel::iospace::record::Kernel::io_for_each_map` header bb4
callee `<core::ops::Range<usize> as Iterator>::find` in `microkernel::iospace::record::Kernel::io_free_dev`
loop `microkernel::iospace::record::Kernel::io_next_map_of_pa` header bb2
loop `microkernel::iospace::record::Kernel::io_stuck_flagged` header bb2
loop `microkernel::iospace::record::Kernel::io_table_dying` header bb7
loop `microkernel::iospace::record::Kernel::io_table_release` header bb9
loop `microkernel::iospace::record::Kernel::io_total_devs` header bb2
callee `<Option<IoHome> as Try>::branch` in `microkernel::iospace::record::Kernel::iospace_frame`
callee `<Result<u64, iospace::IoError> as Try>::branch` in `microkernel::iospace::record::Kernel::iospace_install_table`
callee `<Option<(usize, u32)> as Try>::branch` in `microkernel::iospace::record::Kernel::iospace_missing_level`
callee `<Option<Option<u8>> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::iospace::record::Kernel::iospace_missing_level`
callee `<Option<synthyrion_capauth::SealedCap> as PartialEq>::eq` in `microkernel::iosys::eq`
callee `Formatter::debug_struct_field3_finish` in `microkernel::iosys::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::iosys::fmt`
callee `<Filter<alloc::vec::IntoIter<ReplySlot>, {closure@crates\microkernel\src\ipcx.rs:870:21: 870:24}> as Iterator>::map` in `microkernel::ipcx::Kernel::call_edges`
callee `<alloc::vec::IntoIter<(u64, u64, u64)> as Iterator>::map` in `microkernel::ipcx::Kernel::call_edges`
callee `<alloc::vec::IntoIter<ReplySlot> as Iterator>::filter` in `microkernel::ipcx::Kernel::call_edges`
callee `<alloc::vec::Vec<(u64, u64, u64)> as DerefMut>::deref_mut` in `microkernel::ipcx::Kernel::call_edges`
callee `<alloc::vec::Vec<(u64, u64, u64)> as IntoIterator>::into_iter` in `microkernel::ipcx::Kernel::call_edges`
callee `<core::iter::Map<Filter<alloc::vec::IntoIter<ReplySlot>, {closure@crates\microkernel\src\ipcx.rs:870:21: 870:24}>, {closure@crates\microkernel\src\ipcx.rs:871:18: 871:21}> as Iterator>::collect` in `microkernel::ipcx::Kernel::call_edges`
callee `<core::iter::Map<alloc::vec::IntoIter<(u64, u64, u64)>, {closure@crates\microkernel\src\ipcx.rs:874:31: 874:42}> as Iterator>::collect` in `microkernel::ipcx::Kernel::call_edges`
callee `core::slice::sort_unstable` in `microkernel::ipcx::Kernel::call_edges`
callee `<CallState as PartialEq>::eq` in `microkernel::ipcx::Kernel::call_edges::{closure#0}::{closure#0}`
callee `<[u64; 4] as IntoIterator>::into_iter` in `microkernel::ipcx::Kernel::create_notification_inner`
callee `<core::array::IntoIter<u64, 4> as Iterator>::next` in `microkernel::ipcx::Kernel::create_notification_inner`
loop `microkernel::ipcx::Kernel::create_notification_inner` header bb6
callee `Rights::contains` in `microkernel::ipcx::Kernel::cs_bind_notification`
callee `Rights::contains` in `microkernel::ipcx::Kernel::cs_call`
callee `Rights::contains` in `microkernel::ipcx::Kernel::cs_call_fast`
callee `<Result<XferList, cspace::CsError> as Try>::branch` in `microkernel::ipcx::Kernel::cs_call_xfer`
callee `<Result<synthyrion_abi::SysResult, cspace::CsError> as FromResidual<Result<Infallible, cspace::CsError>>>::from_residual` in `microkernel::ipcx::Kernel::cs_call_xfer`
callee `Rights::contains` in `microkernel::ipcx::Kernel::cs_call_xfer`
callee `<Result<XferList, cspace::CsError> as Try>::branch` in `microkernel::ipcx::Kernel::cs_reply_xfer`
callee `<Result<synthyrion_abi::SysResult, cspace::CsError> as FromResidual<Result<Infallible, cspace::CsError>>>::from_residual` in `microkernel::ipcx::Kernel::cs_reply_xfer`
callee `<Result<XferList, cspace::CsError> as Try>::branch` in `microkernel::ipcx::Kernel::cs_send_xfer`
callee `<Result<synthyrion_abi::SysResult, cspace::CsError> as FromResidual<Result<Infallible, cspace::CsError>>>::from_residual` in `microkernel::ipcx::Kernel::cs_send_xfer`
callee `Rights::contains` in `microkernel::ipcx::Kernel::cs_send_xfer`
callee `Rights::contains` in `microkernel::ipcx::Kernel::cs_signal`
callee `Rights::contains` in `microkernel::ipcx::Kernel::cs_unbind_notification`
callee `Rights::contains` in `microkernel::ipcx::Kernel::cs_wait`
callee `<CallState as PartialEq>::eq` in `microkernel::ipcx::Kernel::cs_wait::{closure#0}`
callee `<[u64; 4] as IntoIterator>::into_iter` in `microkernel::ipcx::Kernel::destroy_notification`
callee `<core::array::IntoIter<u64, 4> as Iterator>::next` in `microkernel::ipcx::Kernel::destroy_notification`
loop `microkernel::ipcx::Kernel::destroy_notification` header bb5
loop `microkernel::ipcx::Kernel::destroy_notification` header bb12
loop `microkernel::ipcx::Kernel::forget_task_ipcx` header bb17
callee `<Filter<alloc::vec::IntoIter<ReplySlot>, {closure@crates\microkernel\src\ipcx.rs:1740:53: 1740:56}> as Iterator>::count` in `microkernel::ipcx::Kernel::ipc_invariants`
callee `<alloc::vec::IntoIter<ReplySlot> as Iterator>::count` in `microkernel::ipcx::Kernel::ipc_invariants`
callee `<alloc::vec::IntoIter<ReplySlot> as Iterator>::filter` in `microkernel::ipcx::Kernel::ipc_invariants`
callee `Arguments::from_str` in `microkernel::ipcx::Kernel::ipc_invariants`
callee `Arguments::new` in `microkernel::ipcx::Kernel::ipc_invariants`
callee `BTreeSet::insert` in `microkernel::ipcx::Kernel::ipc_invariants`
callee `BTreeSet::new` in `microkernel::ipcx::Kernel::ipc_invariants`
callee `alloc::fmt::format` in `microkernel::ipcx::Kernel::ipc_invariants`
callee `core::fmt::rt::Argument::new_display` in `microkernel::ipcx::Kernel::ipc_invariants`
callee `must_use` in `microkernel::ipcx::Kernel::ipc_invariants`
loop `microkernel::ipcx::Kernel::ipc_invariants` header bb4
loop `microkernel::ipcx::Kernel::ipc_invariants` header bb70
loop `microkernel::ipcx::Kernel::ipc_invariants` header bb125
callee `<Option<synthyrion_capauth::SealedCap> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::ipcx::Kernel::mint_badged_endpoint`
callee `<core::slice::Iter<'_, pure::Window> as Iterator>::any` in `microkernel::ipcx::Kernel::move_task_partition_sealed`
callee `<Filter<Values<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\ipcx.rs:2038:50: 2038:53}> as Iterator>::count` in `microkernel::ipcx::Kernel::notification_invariants`
callee `<Result<(), String> as FromResidual<Result<Infallible, String>>>::from_residual` in `microkernel::ipcx::Kernel::notification_invariants`
callee `<Result<usize, String> as Try>::branch` in `microkernel::ipcx::Kernel::notification_invariants`
callee `<Take<core::slice::Iter<'_, u64>> as IntoIterator>::into_iter` in `microkernel::ipcx::Kernel::notification_invariants`
callee `<Take<core::slice::Iter<'_, u64>> as Iterator>::next` in `microkernel::ipcx::Kernel::notification_invariants`
callee `<Values<'_, u64, tasktable::Task> as Iterator>::filter` in `microkernel::ipcx::Kernel::notification_invariants`
callee `Arguments::new` in `microkernel::ipcx::Kernel::notification_invariants`
callee `alloc::fmt::format` in `microkernel::ipcx::Kernel::notification_invariants`
callee `core::fmt::rt::Argument::new_debug` in `microkernel::ipcx::Kernel::notification_invariants`
callee `core::fmt::rt::Argument::new_display` in `microkernel::ipcx::Kernel::notification_invariants`
callee `must_use` in `microkernel::ipcx::Kernel::notification_invariants`
loop `microkernel::ipcx::Kernel::notification_invariants` header bb3
callee `Arguments::new` in `microkernel::ipcx::Kernel::notification_invariants::{closure#0}`
callee `alloc::fmt::format` in `microkernel::ipcx::Kernel::notification_invariants::{closure#0}`
callee `core::fmt::rt::Argument::new_display` in `microkernel::ipcx::Kernel::notification_invariants::{closure#0}`
callee `must_use` in `microkernel::ipcx::Kernel::notification_invariants::{closure#0}`
callee `<String as From<&str>>::from` in `microkernel::ipcx::Kernel::notification_invariants::{closure#1}`
callee `<alloc::vec::IntoIter<ReplySlot> as Iterator>::count` in `microkernel::ipcx::Kernel::open_calls`
callee `<FilterMap<Values<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\ipcx.rs:1040:40: 1040:43}> as Iterator>::collect` in `microkernel::ipcx::Kernel::replies_iter`
callee `<Values<'_, u64, tasktable::Task> as Iterator>::filter_map` in `microkernel::ipcx::Kernel::replies_iter`
callee `<alloc::vec::Vec<ReplySlot> as IntoIterator>::into_iter` in `microkernel::ipcx::Kernel::replies_iter`
callee `VecDeque::retain` in `microkernel::ipcx::Kernel::retire_one`
callee `<Result<&Made, objects::ObjError> as Try>::branch` in `microkernel::ipcx::Kernel::retype_badged_endpoint`
callee `<Result<synthyrion_capauth::SealedCap, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::ipcx::Kernel::retype_badged_endpoint`
callee `<Result<alloc::vec::Vec<Made>, objects::ObjError> as Try>::branch` in `microkernel::ipcx::Kernel::retype_badged_endpoint`
callee `<alloc::vec::Vec<Made> as Deref>::deref` in `microkernel::ipcx::Kernel::retype_badged_endpoint`
callee `<Result<&Made, objects::ObjError> as Try>::branch` in `microkernel::ipcx::Kernel::retype_signal_slot`
callee `<Result<synthyrion_capauth::SealedCap, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::ipcx::Kernel::retype_signal_slot`
callee `<Result<alloc::vec::Vec<Made>, objects::ObjError> as Try>::branch` in `microkernel::ipcx::Kernel::retype_signal_slot`
callee `<alloc::vec::Vec<Made> as Deref>::deref` in `microkernel::ipcx::Kernel::retype_signal_slot`
callee `<Option<synthyrion_capauth::SealedCap> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::ipcx::Kernel::sealed_signal_cap`
callee `<Option<ipcx::CallResult> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::ipcx::Kernel::take_call_result`
callee `core::mem::take` in `microkernel::ipcx::Kernel::take_call_result::{closure#0}`
callee `<KernelX as Default>::default` in `microkernel::ipcx::KernelX::sized`
callee `<CallState as Clone>::clone` in `microkernel::ipcx::clone`
callee `<CapList as Clone>::clone` in `microkernel::ipcx::clone`
callee `<Option<(bool, bool)> as Clone>::clone` in `microkernel::ipcx::clone`
callee `<Option<(u64, usize, u8)> as Clone>::clone` in `microkernel::ipcx::clone`
callee `<Option<Option<u64>> as Clone>::clone` in `microkernel::ipcx::clone`
callee `<Option<ReplySlot> as Clone>::clone` in `microkernel::ipcx::clone`
callee `<Option<[u64; 18]> as Clone>::clone` in `microkernel::ipcx::clone`
callee `<Option<synthyrion_capauth::SealedCap> as Clone>::clone` in `microkernel::ipcx::clone`
callee `<Option<ipcx::Fault> as Clone>::clone` in `microkernel::ipcx::clone`
callee `<Option<u64> as Clone>::clone` in `microkernel::ipcx::clone`
callee `<TaskXGrants as Clone>::clone` in `microkernel::ipcx::clone`
callee `<XferList as Clone>::clone` in `microkernel::ipcx::clone`
callee `<synthyrion_capauth::SealedCap as Clone>::clone` in `microkernel::ipcx::clone`
callee `<ipcx::Fault as Clone>::clone` in `microkernel::ipcx::clone`
callee `<BoundedIdx<u64> as Default>::default` in `microkernel::ipcx::default`
callee `<CallState as Default>::default` in `microkernel::ipcx::default`
callee `<IrqCtl as Default>::default` in `microkernel::ipcx::default`
callee `<Option<(bool, bool)> as Default>::default` in `microkernel::ipcx::default`
callee `<Option<(u64, usize, u8)> as Default>::default` in `microkernel::ipcx::default`
callee `<Option<Option<u64>> as Default>::default` in `microkernel::ipcx::default`
callee `<Option<ReplySlot> as Default>::default` in `microkernel::ipcx::default`
callee `<Option<[u64; 18]> as Default>::default` in `microkernel::ipcx::default`
callee `<Option<synthyrion_capauth::SealedCap> as Default>::default` in `microkernel::ipcx::default`
callee `<Option<ipcx::Fault> as Default>::default` in `microkernel::ipcx::default`
callee `<Option<u64> as Default>::default` in `microkernel::ipcx::default`
callee `<TaskXGrants as Default>::default` in `microkernel::ipcx::default`
callee `<XferList as Default>::default` in `microkernel::ipcx::default`
callee `<u32 as Default>::default` in `microkernel::ipcx::default`
callee `<u64 as Default>::default` in `microkernel::ipcx::default`
callee `<u8 as Default>::default` in `microkernel::ipcx::default`
callee `BTab::new` in `microkernel::ipcx::default`
callee `Box::new` in `microkernel::ipcx::default`
callee `<&CapList as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<&XferList as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<&[synthyrion_capauth::SealedCap] as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<&synthyrion_capauth::SealedCap as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<&ipcx::CallResult as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<&ipcx::Fault as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<&u32 as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<&u64 as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<[XferCap; 3] as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<[synthyrion_capauth::SealedCap; 3] as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<synthyrion_caps::Rights as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `<ipcx::FaultKind as PartialEq>::eq` in `microkernel::ipcx::eq`
callee `alloc::vec::Vec::as_slice` in `microkernel::ipcx::eq`
callee `array::as_slice` in `microkernel::ipcx::eq`
callee `Formatter::debug_struct_field1_finish` in `microkernel::ipcx::fmt`
callee `Formatter::debug_struct_field2_finish` in `microkernel::ipcx::fmt`
callee `Formatter::debug_struct_field3_finish` in `microkernel::ipcx::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::ipcx::fmt`
callee `Formatter::debug_struct_field5_finish` in `microkernel::ipcx::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::ipcx::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::ipcx::fmt`
callee `Formatter::write_str` in `microkernel::ipcx::fmt`
callee `<core::slice::Iter<'_, synthyrion_capauth::SealedCap> as IntoIterator>::into_iter` in `microkernel::ipcx::from`
callee `<core::slice::Iter<'_, synthyrion_capauth::SealedCap> as Iterator>::next` in `microkernel::ipcx::from`
loop `microkernel::ipcx::from` header bb3
callee `Rights::contains` in `microkernel::irqctl::Kernel::cs_control_ok`
callee `Rights::contains` in `microkernel::irqctl::Kernel::cs_handler_line`
callee `Rights::contains` in `microkernel::irqctl::Kernel::cs_irq_handler_set_notification`
callee `BTreeSet::remove` in `microkernel::irqctl::Kernel::irq_ack_line`
callee `BTreeSet::remove` in `microkernel::irqctl::Kernel::irq_clear_line`
callee `alloc::collections::BTreeMap::remove` in `microkernel::irqctl::Kernel::irq_clear_line`
callee `BTreeSet::contains` in `microkernel::irqctl::Kernel::irq_fired_handler`
callee `BTreeSet::insert` in `microkernel::irqctl::Kernel::irq_fired_handler`
callee `alloc::collections::BTreeMap::get` in `microkernel::irqctl::Kernel::irq_fired_handler`
callee `alloc::collections::BTreeMap::remove` in `microkernel::irqctl::Kernel::irq_fired_handler`
callee `alloc::collections::BTreeMap::contains_key` in `microkernel::irqctl::Kernel::irq_handler_bound`
callee `<Result<(u64, u64), synthyrion_abi::SysResult> as Try>::branch` in `microkernel::irqctl::Kernel::irq_handler_get`
callee `<Result<synthyrion_capauth::SealedCap, synthyrion_abi::SysResult> as FromResidual<Result<Infallible, synthyrion_abi::SysResult>>>::from_residual` in `microkernel::irqctl::Kernel::irq_handler_get`
callee `<Result<u64, synthyrion_abi::SysResult> as Try>::branch` in `microkernel::irqctl::Kernel::irq_handler_get`
callee `alloc::collections::BTreeMap::contains_key` in `microkernel::irqctl::Kernel::irq_handler_issued`
callee `alloc::collections::BTreeMap::contains_key` in `microkernel::irqctl::Kernel::irq_issue`
callee `alloc::collections::BTreeMap::insert` in `microkernel::irqctl::Kernel::irq_issue`
callee `BTreeSet::contains` in `microkernel::irqctl::Kernel::irq_masked`
callee `BTreeSet::remove` in `microkernel::irqctl::Kernel::irq_revoke_line`
callee `alloc::collections::BTreeMap::remove` in `microkernel::irqctl::Kernel::irq_revoke_line`
callee `alloc::collections::BTreeMap::contains_key` in `microkernel::irqctl::Kernel::irq_set`
callee `alloc::collections::BTreeMap::insert` in `microkernel::irqctl::Kernel::irq_set`
callee `alloc::collections::BTreeMap::get` in `microkernel::irqctl::Kernel::live_handler_line`
callee `<BTreeSet<u64> as Clone>::clone` in `microkernel::irqctl::clone`
callee `<alloc::collections::BTreeMap<u64, (u64, u64)> as Clone>::clone` in `microkernel::irqctl::clone`
callee `<alloc::collections::BTreeMap<u64, u64> as Clone>::clone` in `microkernel::irqctl::clone`
callee `BTreeSet::new` in `microkernel::irqctl::default`
callee `Formatter::debug_struct_fields_finish` in `microkernel::irqctl::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::irqctl::fmt`
callee `Formatter::write_str` in `microkernel::irqctl::fmt`
callee `core::slice::contains` in `microkernel::is_reserved_authority`
callee `<C as KidGuard>::len` in `microkernel::kids_agree`
callee `<C as KidGuard>::raw` in `microkernel::kids_agree`
loop `microkernel::kids_agree` header bb5
callee `<u32 as Default>::default` in `microkernel::limits::default`
callee `Formatter::debug_struct_fields_finish` in `microkernel::limits::fmt`
callee `Formatter::write_str` in `microkernel::limits::fmt`
callee `<u64 as Default>::default` in `microkernel::linktree::default`
callee `Formatter::debug_struct_field4_finish` in `microkernel::linktree::fmt`
callee `<Filter<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}>, {closure@crates\microkernel\src\mapchain.rs:140:47: 140:50}> as Iterator>::filter_map` in `microkernel::mapchain::Kernel::legacy_window_arena_frames`
callee `<FilterMap<Filter<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}>, {closure@crates\microkernel\src\mapchain.rs:140:47: 140:50}>, {closure@crates\microkernel\src\mapchain.rs:140:137: 140:140}> as Iterator>::map` in `microkernel::mapchain::Kernel::legacy_window_arena_frames`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as Iterator>::filter` in `microkernel::mapchain::Kernel::legacy_window_arena_frames`
callee `<core::iter::Map<FilterMap<Filter<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}>, {closure@crates\microkernel\src\mapchain.rs:140:47: 140:50}>, {closure@crates\microkernel\src\mapchain.rs:140:137: 140:140}>, {closure@crates\microkernel\src\mapchain.rs:140:167: 140:170}> as Iterator>::fold usize {core::num::saturating_add}>` in `microkernel::mapchain::Kernel::legacy_window_arena_frames`
callee `<Option<SpaceBirth> as PartialEq>::ne` in `microkernel::mapchain::Kernel::legacy_window_arena_frames::{closure#0}`
callee `AddressSpace::recorded_pair_count` in `microkernel::mapchain::Kernel::legacy_window_arena_frames::{closure#2}`
callee `AddressSpace::table_supply_len` in `microkernel::mapchain::Kernel::map_rec_available::{closure#0}`
loop `microkernel::mapchain::Kernel::map_rec_drop_root` header bb6
callee `AddressSpace::table_supply_len` in `microkernel::mapchain::Kernel::map_rec_drop_root::{closure#1}`
callee `AddressSpace::table_supply_len` in `microkernel::mapchain::Kernel::map_rec_take::{closure#2}`
callee `AddressSpace::take_spare_supply_frame` in `microkernel::mapchain::Kernel::map_rec_take::{closure#3}`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as IntoIterator>::into_iter` in `microkernel::mapchain::Kernel::mapper_record_frames`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as Iterator>::next` in `microkernel::mapchain::Kernel::mapper_record_frames`
callee `FrameAllocator::used` in `microkernel::mapchain::Kernel::mapper_record_frames`
loop `microkernel::mapchain::Kernel::mapper_record_frames` header bb4
loop `microkernel::mapchain::Kernel::region_backs` header bb9
loop `microkernel::mapchain::Kernel::region_drop_task` header bb4
loop `microkernel::mapchain::Kernel::region_find` header bb8
loop `microkernel::mapchain::Kernel::region_unlink` header bb26
callee `<Option<alloc::vec::Vec<(u64, u64)>> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::mapchain::Kernel::regions_in_map_order`
callee `alloc::vec::Vec::push` in `microkernel::mapchain::Kernel::regions_in_map_order`
callee `core::slice::reverse` in `microkernel::mapchain::Kernel::regions_in_map_order`
loop `microkernel::mapchain::Kernel::regions_in_map_order` header bb9
callee `<PageSize as PartialEq>::eq` in `microkernel::mapchain::eq`
callee `Formatter::debug_struct_field5_finish` in `microkernel::mapchain::fmt`
callee `<Option<ModelRam> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::modelram::ModelRam::try_new_at`
callee `alloc::vec::Vec::resize` in `microkernel::modelram::ModelRam::try_new_at`
callee `alloc::vec::Vec::try_reserve_exact` in `microkernel::modelram::ModelRam::try_new_at`
loop `microkernel::modelram::read` header bb7
callee `<Option<Box<dyn userwalk::PhysMem>> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::modelram::sub_window`
callee `<Option<ModelRam> as Try>::branch` in `microkernel::modelram::sub_window`
callee `Box::new` in `microkernel::modelram::sub_window`
callee `<Enumerate<core::slice::Iter<'_, u8>> as IntoIterator>::into_iter` in `microkernel::modelram::write`
callee `<Enumerate<core::slice::Iter<'_, u8>> as Iterator>::next` in `microkernel::modelram::write`
callee `<core::slice::Iter<'_, u8> as Iterator>::enumerate` in `microkernel::modelram::write`
loop `microkernel::modelram::write` header bb7
callee `<Option<ObjJob> as Default>::default` in `microkernel::objcontain::default`
callee `Formatter::debug_struct_field1_finish` in `microkernel::objcontain::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::objcontain::fmt`
callee `Formatter::write_str` in `microkernel::objcontain::fmt`
callee `<&[(u64, u64)] as IntoIterator>::into_iter` in `microkernel::objects::Kernel::anon_undo`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::objects::Kernel::anon_undo`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::anon_undo`
callee `<core::iter::Map<core::ops::Range<usize>, {closure@crates\microkernel\src\objects.rs:3952:48: 3952:51}> as Iterator>::collect` in `microkernel::objects::Kernel::anon_undo`
callee `<core::ops::Range<usize> as Iterator>::map` in `microkernel::objects::Kernel::anon_undo`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::next` in `microkernel::objects::Kernel::anon_undo`
callee `<core::slice::Iter<'_, u64> as Iterator>::any` in `microkernel::objects::Kernel::anon_undo`
loop `microkernel::objects::Kernel::anon_undo` header bb2
loop `microkernel::objects::Kernel::anon_undo` header bb19
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::objects::Kernel::anon_undo::{closure#1}`
callee `<DomainIsolation as PartialEq>::eq` in `microkernel::objects::Kernel::arm_domain_isolation`
callee `<Result<(), objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::objects::Kernel::asid_assign`
callee `<Option<Payload> as PartialEq>::ne` in `microkernel::objects::Kernel::asid_assign_id`
callee `<Option<Payload> as PartialEq>::ne` in `microkernel::objects::Kernel::asid_pool_make`
callee `<Result<synthyrion_capauth::SealedCap, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::objects::Kernel::asid_pool_make`
callee `<alloc::vec::IntoIter<synthyrion_capauth::SealedCap> as Iterator>::last` in `microkernel::objects::Kernel::asid_pool_make`
callee `<alloc::vec::Vec<synthyrion_capauth::SealedCap> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::asid_pool_make`
callee `core::ops::Range::contains` in `microkernel::objects::Kernel::asid_pool_make`
callee `<Result<(), objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::objects::Kernel::bind_sched_context`
callee `<Filter<core::slice::Iter<'_, u64>, {closure@crates\microkernel\src\objects.rs:3303:50: 3303:53}> as Iterator>::count` in `microkernel::objects::Kernel::boot_give_space`
callee `<Result<u64, synthyrion_abi::SysResult> as FromResidual<Result<Infallible, synthyrion_abi::SysResult>>>::from_residual` in `microkernel::objects::Kernel::boot_give_space`
callee `<Result<u64, synthyrion_abi::SysResult> as Try>::branch` in `microkernel::objects::Kernel::boot_give_space`
callee `<alloc::vec::IntoIter<(u64, u64)> as Iterator>::next` in `microkernel::objects::Kernel::boot_give_space`
callee `<alloc::vec::Vec<(u64, u64)> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::boot_give_space`
callee `<alloc::vec::Vec<(u64, usize, u64)> as Extend<(u64, usize, u64)>>::extend` in `microkernel::objects::Kernel::boot_give_space`
callee `<core::slice::Iter<'_, (usize, u64)> as Iterator>::any` in `microkernel::objects::Kernel::boot_give_space`
callee `<core::slice::Iter<'_, (usize, u64)> as Iterator>::map` in `microkernel::objects::Kernel::boot_give_space`
callee `<core::slice::Iter<'_, u64> as Iterator>::filter` in `microkernel::objects::Kernel::boot_give_space`
callee `Option::get_or_insert_with Box<ArchHooks> {<Box<ArchHooks> as Default>::default}>` in `microkernel::objects::Kernel::boot_give_space`
callee `alloc::vec::Vec::extend_from_slice` in `microkernel::objects::Kernel::boot_give_space`
callee `alloc::vec::Vec::push` in `microkernel::objects::Kernel::boot_give_space`
callee `alloc::vec::Vec::with_capacity` in `microkernel::objects::Kernel::boot_give_space`
loop `microkernel::objects::Kernel::boot_give_space` header bb32
loop `microkernel::objects::Kernel::boot_give_space` header bb83
callee `AddressSpace::provide_table_frame` in `microkernel::objects::Kernel::boot_give_space::{closure#5}`
callee `AddressSpace::add_table_frames` in `microkernel::objects::Kernel::boot_give_space::{closure#6}`
callee `<alloc::vec::Vec<Made> as Deref>::deref` in `microkernel::objects::Kernel::carve_sidecar`
callee `AddressSpace::child_leaf_pa` in `microkernel::objects::Kernel::child_leaf_frozen`
callee `AddressSpace::child_table` in `microkernel::objects::Kernel::child_leaf_frozen`
callee `<Result<(), objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::objects::Kernel::configure_sched_context_on`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::any` in `microkernel::objects::Kernel::create_device_untyped`
callee `FrameAllocator::total` in `microkernel::objects::Kernel::create_device_untyped`
callee `alloc::vec::Vec::push` in `microkernel::objects::Kernel::create_device_untyped`
loop `microkernel::objects::Kernel::create_device_untyped` header bb15
loop `microkernel::objects::Kernel::create_device_untyped` header bb33
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::objects::Kernel::create_device_untyped::{closure#2}`
callee `<Option<Frame> as Try>::branch` in `microkernel::objects::Kernel::create_root_untyped`
callee `<Option<synthyrion_capauth::SealedCap> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::objects::Kernel::create_root_untyped`
callee `FrameAllocator::alloc_contiguous` in `microkernel::objects::Kernel::create_root_untyped`
loop `microkernel::objects::Kernel::create_root_untyped` header bb20
loop `microkernel::objects::Kernel::create_root_untyped` header bb36
callee `<Result<(synthyrion_caps::ObjKind, u64, synthyrion_caps::Rights, u64), objects::MapError> as Try>::branch` in `microkernel::objects::Kernel::cs_map_frame`
callee `Rights::contains` in `microkernel::objects::Kernel::cs_map_frame`
callee `<Result<usize, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::objects::Kernel::destroy_root_untyped`
loop `microkernel::objects::Kernel::destroy_root_untyped` header bb18
loop `microkernel::objects::Kernel::destroy_root_untyped` header bb26
callee `<Zip<Zip<core::slice::Iter<'_, u64>, core::slice::Iter<'_, Option<u8>>>, core::slice::Iter<'_, u64>> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::effective_priority`
callee `<Zip<Zip<core::slice::Iter<'_, u64>, core::slice::Iter<'_, Option<u8>>>, core::slice::Iter<'_, u64>> as Iterator>::next` in `microkernel::objects::Kernel::effective_priority`
callee `<Zip<core::slice::Iter<'_, u64>, core::slice::Iter<'_, Option<u8>>> as Iterator>::zip` in `microkernel::objects::Kernel::effective_priority`
callee `<alloc::vec::Vec<Option<u8>> as Deref>::deref` in `microkernel::objects::Kernel::effective_priority`
loop `microkernel::objects::Kernel::effective_priority` header bb11
callee `<Result<synthyrion_vmm::cachemaint::FlushPlan, objects::FlushError> as FromResidual<Result<Infallible, objects::FlushError>>>::from_residual` in `microkernel::objects::Kernel::flush_range`
callee `<Result<u64, objects::FlushError> as Try>::branch` in `microkernel::objects::Kernel::flush_range`
callee `<Result<Obj, objects::FlushError> as Try>::branch` in `microkernel::objects::Kernel::flush_range_id`
callee `<Result<synthyrion_vmm::cachemaint::FlushPlan, objects::FlushError> as FromResidual<Result<Infallible, objects::FlushError>>>::from_residual` in `microkernel::objects::Kernel::flush_range_id`
callee `flush_plan` in `microkernel::objects::Kernel::flush_range_id`
callee `<Option<(u64, u64, PageSize)> as Try>::branch` in `microkernel::objects::Kernel::frame_frozen`
callee `<Option<(u64, u64, PageSize)> as Try>::branch` in `microkernel::objects::Kernel::frame_mapping_counts`
callee `<Option<(usize, usize)> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::objects::Kernel::frame_mapping_counts`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::objects::Kernel::frame_obj_by_phys`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::frame_obj_by_phys`
loop `microkernel::objects::Kernel::frame_obj_by_phys` header bb3
callee `<Result<objects::Unmapped, objects::MapError> as FromResidual<Result<Infallible, objects::MapError>>>::from_residual` in `microkernel::objects::Kernel::frame_unmap`
callee `AddressSpace::translate` in `microkernel::objects::Kernel::frame_unmap_obj::{closure#0}`
callee `AddressSpace::unmap` in `microkernel::objects::Kernel::frame_unmap_obj::{closure#2}`
callee `AddressSpace::unmap_large` in `microkernel::objects::Kernel::frame_unmap_obj::{closure#2}`
loop `microkernel::objects::Kernel::hdr_back_frame` header bb4
callee `<ObjMut<'_> as Deref>::deref` in `microkernel::objects::Kernel::hdr_carve`
callee `<Option<(u64, usize, u64)> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::objects::Kernel::hdr_mark`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::objects::Kernel::hdr_release_all`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::hdr_release_all`
callee `<core::iter::Map<core::ops::Range<usize>, {closure@crates\microkernel\src\objects.rs:3979:50: 3979:53}> as Iterator>::collect` in `microkernel::objects::Kernel::hdr_release_all`
callee `<core::ops::Range<usize> as Iterator>::map` in `microkernel::objects::Kernel::hdr_release_all`
callee `<core::slice::Iter<'_, u64> as Iterator>::any` in `microkernel::objects::Kernel::hdr_release_all`
loop `microkernel::objects::Kernel::hdr_release_all` header bb14
callee `call (1 closure position(s) seen but 0 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::objects::Kernel::hdr_release_all::{closure#1}`
loop `microkernel::objects::Kernel::hdr_reserve` header bb2
callee `<Payload as PartialEq>::eq` in `microkernel::objects::Kernel::is_live_asid_control::{closure#0}`
callee `<alloc::vec::IntoIter<u64> as Iterator>::any` in `microkernel::objects::Kernel::is_retyped_task`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::is_retyped_task`
callee `<ObjMut<'_> as Deref>::deref` in `microkernel::objects::Kernel::kill_object::{closure#1}`
callee `<Filter<Values<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\objects.rs:2915:21: 2915:24}> as Iterator>::map` in `microkernel::objects::Kernel::live_partitions`
callee `<Values<'_, u64, tasktable::Task> as Iterator>::filter` in `microkernel::objects::Kernel::live_partitions`
callee `<core::iter::Map<Filter<Values<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\objects.rs:2915:21: 2915:24}>, {closure@crates\microkernel\src\objects.rs:2916:18: 2916:21}> as Iterator>::collect` in `microkernel::objects::Kernel::live_partitions`
callee `<Enumerate<core::slice::Iter<'_, (u64, u64)>> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::map_anon_ids`
callee `<Enumerate<core::slice::Iter<'_, (u64, u64)>> as Iterator>::next` in `microkernel::objects::Kernel::map_anon_ids`
callee `<alloc::vec::Vec<(u64, u64)> as Extend<(u64, u64)>>::extend` in `microkernel::objects::Kernel::map_anon_ids`
callee `<alloc::vec::Vec<Made> as Deref>::deref` in `microkernel::objects::Kernel::map_anon_ids`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::enumerate` in `microkernel::objects::Kernel::map_anon_ids`
callee `<core::slice::Iter<'_, Made> as Iterator>::filter_map` in `microkernel::objects::Kernel::map_anon_ids`
callee `alloc::vec::Vec::with_capacity` in `microkernel::objects::Kernel::map_anon_ids`
loop `microkernel::objects::Kernel::map_anon_ids` header bb19
loop `microkernel::objects::Kernel::map_anon_ids` header bb42
callee `AddressSpace::table_supply_len` in `microkernel::objects::Kernel::map_frame_inner`
callee `AddressSpace::tables_needed` in `microkernel::objects::Kernel::map_frame_inner`
callee `AddressSpace::translate` in `microkernel::objects::Kernel::map_frame_inner`
callee `check_range` in `microkernel::objects::Kernel::map_frame_inner`
callee `AddressSpace::range_occupancy_healing` in `microkernel::objects::Kernel::map_frame_inner::{closure#1}`
callee `AddressSpace::map` in `microkernel::objects::Kernel::map_frame_inner::{closure#2}`
callee `AddressSpace::map_large` in `microkernel::objects::Kernel::map_frame_inner::{closure#3}`
callee `AddressSpace::unmap` in `microkernel::objects::Kernel::map_frame_inner::{closure#4}`
callee `AddressSpace::unmap_large` in `microkernel::objects::Kernel::map_frame_inner::{closure#5}`
callee `AddressSpace::unmap` in `microkernel::objects::Kernel::map_frame_inner::{closure#6}`
callee `AddressSpace::unmap_large` in `microkernel::objects::Kernel::map_frame_inner::{closure#7}`
callee `<ObjMut<'_> as Deref>::deref` in `microkernel::objects::Kernel::pop_dead_tail_one`
callee `<alloc::vec::Vec<u64> as Extend<u64>>::extend` in `microkernel::objects::Kernel::pop_dead_tail_one`
callee `Box::new_uninit` in `microkernel::objects::Kernel::pop_dead_tail_one`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `microkernel::objects::Kernel::pop_dead_tail_one`
callee `alloc::vec::Vec::pop` in `microkernel::objects::Kernel::pop_dead_tail_one`
loop `microkernel::objects::Kernel::pop_dead_tail_one` header bb7
callee `<Zip<Zip<core::slice::Iter<'_, u64>, core::slice::Iter<'_, Option<u8>>>, core::slice::Iter<'_, u64>> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::priority_pick_spec`
callee `<Zip<Zip<core::slice::Iter<'_, u64>, core::slice::Iter<'_, Option<u8>>>, core::slice::Iter<'_, u64>> as Iterator>::next` in `microkernel::objects::Kernel::priority_pick_spec`
callee `<Zip<core::slice::Iter<'_, u64>, core::slice::Iter<'_, Option<u8>>> as Iterator>::zip` in `microkernel::objects::Kernel::priority_pick_spec`
callee `<alloc::vec::Vec<(Option<u8>, bool, u64)> as Deref>::deref` in `microkernel::objects::Kernel::priority_pick_spec`
callee `<alloc::vec::Vec<Option<u8>> as Deref>::deref` in `microkernel::objects::Kernel::priority_pick_spec`
callee `alloc::vec::Vec::push` in `microkernel::objects::Kernel::priority_pick_spec`
callee `alloc::vec::Vec::with_capacity` in `microkernel::objects::Kernel::priority_pick_spec`
loop `microkernel::objects::Kernel::priority_pick_spec` header bb13
callee `<Copied<Keys<'_, u64, tasktable::Task>> as Iterator>::collect` in `microkernel::objects::Kernel::priority_view`
callee `<FilterMap<alloc::vec::IntoIter<(u64, u64)>, {closure@crates\microkernel\src\objects.rs:2170:54: 2170:62}> as Iterator>::collect` in `microkernel::objects::Kernel::priority_view`
callee `<Keys<'_, u64, tasktable::Task> as Iterator>::copied` in `microkernel::objects::Kernel::priority_view`
callee `<Values<'_, u64, tasktable::Task> as Iterator>::map` in `microkernel::objects::Kernel::priority_view`
callee `<alloc::vec::IntoIter<(u64, u64)> as Iterator>::filter_map` in `microkernel::objects::Kernel::priority_view`
callee `<alloc::vec::Vec<(u64, u64)> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::priority_view`
callee `<alloc::vec::Vec<(usize, usize)> as Deref>::deref` in `microkernel::objects::Kernel::priority_view`
callee `<alloc::vec::Vec<Option<u8>> as Deref>::deref` in `microkernel::objects::Kernel::priority_view`
callee `<core::iter::Map<Values<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\objects.rs:2163:60: 2163:63}> as Iterator>::collect` in `microkernel::objects::Kernel::priority_view`
callee `<core::iter::Map<Values<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\objects.rs:2167:18: 2167:21}> as Iterator>::collect` in `microkernel::objects::Kernel::priority_view`
callee `<Option<(usize, usize)> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::objects::Kernel::priority_view::{closure#2}`
callee `core::slice::binary_search` in `microkernel::objects::Kernel::priority_view::{closure#2}`
callee `AddressSpace::for_each_leaf` in `microkernel::objects::Kernel::real_leaves`
callee `RoView::health` in `microkernel::objects::Kernel::real_leaves`
callee `<PageSize as PartialEq>::eq` in `microkernel::objects::Kernel::real_leaves::{closure#0}`
callee `alloc::vec::Vec::push` in `microkernel::objects::Kernel::real_leaves::{closure#0}`
callee `<alloc::vec::IntoIter<u64> as Iterator>::map` in `microkernel::objects::Kernel::real_table_image`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::real_table_image`
callee `<core::iter::Map<alloc::vec::IntoIter<u64>, {closure@crates\microkernel\src\objects.rs:3398:18: 3398:24}> as Iterator>::collect` in `microkernel::objects::Kernel::real_table_image`
callee `AddressSpace::table_frames` in `microkernel::objects::Kernel::real_table_image`
callee `RoView::health` in `microkernel::objects::Kernel::real_table_image`
callee `<alloc::vec::IntoIter<(usize, u64)> as Iterator>::map` in `microkernel::objects::Kernel::real_table_image::{closure#0}`
callee `<alloc::vec::IntoIter<(usize, u64)> as Iterator>::next` in `microkernel::objects::Kernel::real_table_image::{closure#0}`
callee `<alloc::vec::Vec<(usize, u64)> as DerefMut>::deref_mut` in `microkernel::objects::Kernel::real_table_image::{closure#0}`
callee `<alloc::vec::Vec<(usize, u64)> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::real_table_image::{closure#0}`
callee `<core::iter::Map<alloc::vec::IntoIter<(usize, u64)>, {closure@crates\microkernel\src\objects.rs:3402:26: 3402:34}> as Iterator>::collect` in `microkernel::objects::Kernel::real_table_image::{closure#0}`
callee `AddressSpace::table_present_entries` in `microkernel::objects::Kernel::real_table_image::{closure#0}`
callee `alloc::vec::Vec::push` in `microkernel::objects::Kernel::real_table_image::{closure#0}`
callee `alloc::vec::Vec::retain` in `microkernel::objects::Kernel::real_table_image::{closure#0}`
callee `core::slice::sort_unstable_by_key` in `microkernel::objects::Kernel::real_table_image::{closure#0}`
loop `microkernel::objects::Kernel::real_table_image::{closure#0}` header bb9
loop `microkernel::objects::Kernel::reclaim_dead_tail` header bb1
callee `<Result<alloc::vec::Vec<Made>, objects::ObjError> as Try>::branch` in `microkernel::objects::Kernel::retype`
callee `<Result<alloc::vec::Vec<synthyrion_capauth::SealedCap>, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::objects::Kernel::retype`
callee `<alloc::vec::IntoIter<Made> as Iterator>::map` in `microkernel::objects::Kernel::retype`
callee `<alloc::vec::Vec<Made> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::retype`
callee `<core::iter::Map<alloc::vec::IntoIter<Made>, {closure@crates\microkernel\src\objects.rs:685:33: 685:36}> as Iterator>::collect` in `microkernel::objects::Kernel::retype`
callee `<ObjMut<'_> as Deref>::deref` in `microkernel::objects::Kernel::retype_in_for`
callee `<Result<alloc::vec::Vec<Made>, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::objects::Kernel::retype_in_for`
callee `<alloc::vec::Vec<usize> as Deref>::deref` in `microkernel::objects::Kernel::retype_in_for`
callee `<alloc::vec::Vec<usize> as DerefMut>::deref_mut` in `microkernel::objects::Kernel::retype_in_for`
callee `<core::slice::Iter<'_, usize> as Iterator>::copied` in `microkernel::objects::Kernel::retype_in_for`
callee `alloc::vec::Vec::push` in `microkernel::objects::Kernel::retype_in_for`
callee `alloc::vec::Vec::with_capacity` in `microkernel::objects::Kernel::retype_in_for`
callee `alloc::vec::from_elem` in `microkernel::objects::Kernel::retype_in_for`
callee `core::ops::Range::contains` in `microkernel::objects::Kernel::retype_in_for`
callee `core::ops::RangeInclusive::contains` in `microkernel::objects::Kernel::retype_in_for`
loop `microkernel::objects::Kernel::retype_in_for` header bb122
loop `microkernel::objects::Kernel::retype_in_for` header bb147
loop `microkernel::objects::Kernel::retype_in_for` header bb269
loop `microkernel::objects::Kernel::retype_in_for` header bb281
loop `microkernel::objects::Kernel::retype_in_for` header bb503
callee `<Payload as PartialEq>::ne` in `microkernel::objects::Kernel::retyped_frame_phys`
callee `alloc::collections::BTreeMap::get` in `microkernel::objects::Kernel::retyped_frame_read`
callee `alloc::collections::BTreeMap::insert` in `microkernel::objects::Kernel::retyped_frame_write`
callee `<ObjMut<'_> as Deref>::deref` in `microkernel::objects::Kernel::revoke_untyped_step`
callee `<alloc::vec::Vec<u64> as Extend<u64>>::extend` in `microkernel::objects::Kernel::revoke_untyped_step`
callee `alloc::vec::Vec::pop` in `microkernel::objects::Kernel::revoke_untyped_step`
callee `core::mem::take` in `microkernel::objects::Kernel::revoke_untyped_step`
loop `microkernel::objects::Kernel::revoke_untyped_step` header bb11
loop `microkernel::objects::Kernel::revoke_untyped_step` header bb61
loop `microkernel::objects::Kernel::sc_notification_destroyed` header bb7
loop `microkernel::objects::Kernel::sc_settle_retiring` header bb10
callee `<Option<synthyrion_capauth::SealedCap> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::objects::Kernel::sched_context_cap`
callee `alloc::collections::BTreeMap::remove` in `microkernel::objects::Kernel::scrub_object`
loop `microkernel::objects::Kernel::scrub_object` header bb5
callee `<Filter<core::slice::Iter<'_, u64>, {closure@crates\microkernel\src\objects.rs:3230:42: 3230:45}> as Iterator>::count` in `microkernel::objects::Kernel::sidecar_room`
callee `<Take<core::slice::Iter<'_, u64>> as Iterator>::any` in `microkernel::objects::Kernel::sidecar_room`
callee `<core::slice::Iter<'_, u64> as Iterator>::filter` in `microkernel::objects::Kernel::sidecar_room`
loop `microkernel::objects::Kernel::sidecar_room` header bb4
callee `<Result<(u64, u64, PageSize), synthyrion_abi::SysResult> as Try>::branch` in `microkernel::objects::Kernel::table_candidate`
callee `<Result<u64, synthyrion_abi::SysResult> as FromResidual<Result<Infallible, synthyrion_abi::SysResult>>>::from_residual` in `microkernel::objects::Kernel::table_candidate`
callee `<Result<(), objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::objects::Kernel::unbind_sched_context`
loop `microkernel::objects::Kernel::unmap_frame_everywhere` header bb13
callee `AddressSpace::translate` in `microkernel::objects::Kernel::unmap_frame_everywhere::{closure#0}`
callee `AddressSpace::unmap` in `microkernel::objects::Kernel::unmap_frame_everywhere::{closure#0}`
callee `AddressSpace::unmap_large` in `microkernel::objects::Kernel::unmap_frame_everywhere::{closure#0}`
callee `<Option<(usize, usize)> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::objects::Kernel::untyped_header_and_mark`
callee `<&alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::untyped_invariants`
callee `<Windows<'_, (usize, usize)> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::untyped_invariants`
callee `<Windows<'_, (usize, usize)> as Iterator>::next` in `microkernel::objects::Kernel::untyped_invariants`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::objects::Kernel::untyped_invariants`
callee `<alloc::vec::Vec<(usize, usize)> as Deref>::deref` in `microkernel::objects::Kernel::untyped_invariants`
callee `<alloc::vec::Vec<(usize, usize)> as DerefMut>::deref_mut` in `microkernel::objects::Kernel::untyped_invariants`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::objects::Kernel::untyped_invariants`
callee `<core::iter::Map<core::slice::Iter<'_, (usize, usize)>, {closure@crates\microkernel\src\objects.rs:1580:45: 1580:48}> as Iterator>::sum` in `microkernel::objects::Kernel::untyped_invariants`
callee `<core::slice::Iter<'_, (usize, usize)> as Iterator>::map` in `microkernel::objects::Kernel::untyped_invariants`
callee `Arguments::new` in `microkernel::objects::Kernel::untyped_invariants`
callee `FrameAllocator::used` in `microkernel::objects::Kernel::untyped_invariants`
callee `alloc::fmt::format` in `microkernel::objects::Kernel::untyped_invariants`
callee `alloc::vec::Vec::push` in `microkernel::objects::Kernel::untyped_invariants`
callee `core::fmt::rt::Argument::new_debug` in `microkernel::objects::Kernel::untyped_invariants`
callee `core::fmt::rt::Argument::new_display` in `microkernel::objects::Kernel::untyped_invariants`
callee `core::fmt::rt::Argument::new_lower_hex` in `microkernel::objects::Kernel::untyped_invariants`
callee `core::slice::sort_unstable` in `microkernel::objects::Kernel::untyped_invariants`
callee `core::slice::windows` in `microkernel::objects::Kernel::untyped_invariants`
callee `must_use` in `microkernel::objects::Kernel::untyped_invariants`
loop `microkernel::objects::Kernel::untyped_invariants` header bb4
loop `microkernel::objects::Kernel::untyped_invariants` header bb107
callee `<Option<synthyrion_capauth::SealedCap> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::objects::Kernel::vcpu_cap`
callee `<Result<(), synthyrion_abi::SysResult> as FromResidual<Result<Infallible, synthyrion_abi::SysResult>>>::from_residual` in `microkernel::objects::Kernel::vspace_donate_table`
callee `<Result<u64, synthyrion_abi::SysResult> as Try>::branch` in `microkernel::objects::Kernel::vspace_donate_table`
callee `AddressSpace::store` in `microkernel::objects::Kernel::vspace_donate_table`
callee `TableBackend::is_charged` in `microkernel::objects::Kernel::vspace_donate_table`
callee `AddressSpace::can_provide_table_frame` in `microkernel::objects::Kernel::vspace_donate_table::{closure#0}`
callee `AddressSpace::provide_table_frame` in `microkernel::objects::Kernel::vspace_donate_table::{closure#1}`
callee `AddressSpace::add_table_frames` in `microkernel::objects::Kernel::vspace_donate_table::{closure#2}`
callee `Option::get_or_insert_with Box<ArchHooks> {<Box<ArchHooks> as Default>::default}>` in `microkernel::objects::Kernel::vspace_place_task`
callee `alloc::vec::Vec::push` in `microkernel::objects::Kernel::vspace_place_task`
callee `BTreeSet::insert` in `microkernel::objects::Kernel::would_split_domains`
callee `<alloc::vec::Vec<u64> as Extend<u64>>::extend` in `microkernel::objects::KernelO::count_descendants`
callee `alloc::vec::Vec::pop` in `microkernel::objects::KernelO::count_descendants`
loop `microkernel::objects::KernelO::count_descendants` header bb1
callee `<KernelO as Default>::default` in `microkernel::objects::KernelO::sized`
callee `<Option<u64> as Clone>::clone` in `microkernel::objects::clone`
callee `<Payload as Clone>::clone` in `microkernel::objects::clone`
callee `<AsidTable as Default>::default` in `microkernel::objects::default`
callee `<BTab<u64, u64, 64> as Default>::default` in `microkernel::objects::default`
callee `<BoundedIdx<u64> as Default>::default` in `microkernel::objects::default`
callee `<ChainStore<Sc, 64, 32, 51> as Default>::default` in `microkernel::objects::default`
callee `<ObjContain as Default>::default` in `microkernel::objects::default`
callee `<ObjMap as Default>::default` in `microkernel::objects::default`
callee `<Option<(u64, u64)> as Default>::default` in `microkernel::objects::default`
callee `<Option<Box<ArchHooks>> as Default>::default` in `microkernel::objects::default`
callee `<Option<u64> as Default>::default` in `microkernel::objects::default`
callee `<bool as Default>::default` in `microkernel::objects::default`
callee `<u64 as Default>::default` in `microkernel::objects::default`
callee `<&PageSize as PartialEq>::eq` in `microkernel::objects::eq`
callee `<&SwitchKind as PartialEq>::eq` in `microkernel::objects::eq`
callee `<&bool as PartialEq>::eq` in `microkernel::objects::eq`
callee `<&u16 as PartialEq>::eq` in `microkernel::objects::eq`
callee `<&u64 as PartialEq>::eq` in `microkernel::objects::eq`
callee `<&u8 as PartialEq>::eq` in `microkernel::objects::eq`
callee `<&usize as PartialEq>::eq` in `microkernel::objects::eq`
callee `<&vcpu::Backend as PartialEq>::eq` in `microkernel::objects::eq`
callee `<Option<u16> as PartialEq>::eq` in `microkernel::objects::eq`
callee `<PageSize as PartialEq>::eq` in `microkernel::objects::eq`
callee `Formatter::debug_struct_field1_finish` in `microkernel::objects::fmt`
callee `Formatter::debug_struct_field2_finish` in `microkernel::objects::fmt`
callee `Formatter::debug_struct_field3_finish` in `microkernel::objects::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::objects::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::objects::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::objects::fmt`
callee `Formatter::write_str` in `microkernel::objects::fmt`
loop `microkernel::objhdr::HdrFrames::free_slot` header bb7
callee `<M as HdrMem>::claim` in `microkernel::objhdr::HdrFrames::init_frame`
loop `microkernel::objhdr::HdrFrames::insert` header bb12
loop `microkernel::objhdr::HdrFrames::locs_into` header bb7
callee `<M as HdrMem>::release` in `microkernel::objhdr::HdrFrames::unlink_frame`
loop `microkernel::objhdr::HdrFrames::unlink_frame` header bb24
callee `<Result<[u64; 13], HdrErr> as FromResidual<Result<Infallible, HdrErr>>>::from_residual` in `microkernel::objhdr::HdrStore::get`
callee `<Result<[u64; 13], HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::get`
callee `<Result<[u64; 32], HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::get`
callee `<Result<u64, HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::get`
callee `<Result<[u64; 32], HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::heal`
callee `<Result<objhdr::Healed, HdrErr> as FromResidual<Result<Infallible, HdrErr>>>::from_residual` in `microkernel::objhdr::HdrStore::heal`
callee `<Result<u64, HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::heal`
callee `<Result<(u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::ids_into`
callee `<Result<(u64, u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::ids_into`
callee `<Result<usize, HdrErr> as FromResidual<Result<Infallible, HdrErr>>>::from_residual` in `microkernel::objhdr::HdrStore::ids_into`
loop `microkernel::objhdr::HdrStore::ids_into` header bb7
callee `<Result<(), HdrErr> as FromResidual<Result<Infallible, HdrErr>>>::from_residual` in `microkernel::objhdr::HdrStore::insert`
callee `<Result<(), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::insert`
callee `<Result<(u64, bool), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::insert`
callee `<Result<(u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::insert`
callee `<Result<(u64, u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::insert`
callee `<Result<u64, HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::insert`
loop `microkernel::objhdr::HdrStore::insert` header bb38
callee `<Result<(u64, u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::len`
callee `<Result<usize, HdrErr> as FromResidual<Result<Infallible, HdrErr>>>::from_residual` in `microkernel::objhdr::HdrStore::len`
callee `<Result<(u64, bool), HdrErr> as FromResidual<Result<Infallible, HdrErr>>>::from_residual` in `microkernel::objhdr::HdrStore::lower`
callee `<Result<(u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::lower`
loop `microkernel::objhdr::HdrStore::lower` header bb3
callee `<Result<(), HdrErr> as FromResidual<Result<Infallible, HdrErr>>>::from_residual` in `microkernel::objhdr::HdrStore::put`
callee `<Result<u64, HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::put`
callee `<Option<&mut u64> as Try>::branch` in `microkernel::objhdr::HdrStore::rd_block`
callee `<Option<[u64; 32]> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::objhdr::HdrStore::rd_block`
loop `microkernel::objhdr::HdrStore::rd_block` header bb3
callee `<(u64, u64, u64, u64) as PartialEq>::eq` in `microkernel::objhdr::HdrStore::rd_meta`
callee `<Result<(), HdrErr> as FromResidual<Result<Infallible, HdrErr>>>::from_residual` in `microkernel::objhdr::HdrStore::remove`
callee `<Result<(), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::remove`
callee `<Result<(u64, bool), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::remove`
callee `<Result<(u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::remove`
callee `<Result<(u64, u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::remove`
loop `microkernel::objhdr::HdrStore::remove` header bb24
callee `<Result<(u64, bool), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::slot_of`
callee `<Result<(u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::slot_of`
callee `<Result<(u64, u64, u64), HdrErr> as Try>::branch` in `microkernel::objhdr::HdrStore::slot_of`
callee `<Result<u64, HdrErr> as FromResidual<Result<Infallible, HdrErr>>>::from_residual` in `microkernel::objhdr::HdrStore::slot_of`
loop `microkernel::objhdr::HdrStore::wr_block` header bb3
callee `<&usize as PartialEq>::eq` in `microkernel::objhdr::eq`
callee `Formatter::debug_struct_field5_finish` in `microkernel::objhdr::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::objhdr::fmt`
callee `Formatter::write_str` in `microkernel::objhdr::fmt`
callee `<ScrubStats as Default>::default` in `microkernel::objheal::default`
callee `<[u64; 2] as Default>::default` in `microkernel::objheal::default`
callee `<u64 as Default>::default` in `microkernel::objheal::default`
callee `<usize as Default>::default` in `microkernel::objheal::default`
callee `Formatter::debug_struct_field3_finish` in `microkernel::objheal::fmt`
callee `Option::insert` in `microkernel::objmem::ObjRam::attach`
callee `Option::replace` in `microkernel::objmem::ObjRam::replace_auto`
callee `Cell::new` in `microkernel::objmem::ObjRam::without_emulation`
callee `<Result<(), Box<dyn userwalk::PhysMem>> as FromResidual<Result<Infallible, Box<dyn userwalk::PhysMem>>>>::from_residual` in `microkernel::objmem::crate::Kernel::attach_object_memory`
callee `<Result<(), Box<dyn userwalk::PhysMem>> as Try>::branch` in `microkernel::objmem::crate::Kernel::attach_object_memory`
loop `microkernel::objstore::ObjHeaders::frame_step` header bb7
callee `<u64 as Default>::default` in `microkernel::objstore::default`
callee `Formatter::debug_struct_fields_finish` in `microkernel::objstore::fmt`
callee `Formatter::write_str` in `microkernel::objstore::fmt`
callee `alloc::vec::Vec::push` in `microkernel::objtab::ObjMap::chain`
loop `microkernel::objtab::ObjMap::chain` header bb4
callee `alloc::vec::Vec::push` in `microkernel::objtab::ObjMap::children`
loop `microkernel::objtab::ObjMap::children` header bb4
callee `<ObjView as Default>::default` in `microkernel::objtab::ObjMap::for_each`
callee `<Take<core::slice::Iter<'_, ObjView>> as Iterator>::next` in `microkernel::objtab::ObjMap::for_each`
callee `<dyn for<'a> FnMut(&'a ObjView) as FnMut<(&ObjView,)>>::call_mut` in `microkernel::objtab::ObjMap::for_each`
loop `microkernel::objtab::ObjMap::for_each` header bb4
callee `<Option<objhdr::Healed> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::objtab::ObjMap::heal`
callee `<Take<core::slice::Iter<'_, u64>> as IntoIterator>::into_iter` in `microkernel::objtab::ObjMap::ids`
callee `<Take<core::slice::Iter<'_, u64>> as Iterator>::next` in `microkernel::objtab::ObjMap::ids`
callee `alloc::vec::Vec::push` in `microkernel::objtab::ObjMap::ids`
callee `core::slice::sort_unstable` in `microkernel::objtab::ObjMap::ids`
loop `microkernel::objtab::ObjMap::ids` header bb4
callee `<dyn for<'a> Fn(&'a ObjView) -> bool as Fn<(&ObjView,)>>::call` in `microkernel::objtab::ObjMap::min_id_where::{closure#0}`
callee `Cell::new` in `microkernel::objtab::ObjMap::new`
callee `alloc::vec::from_elem` in `microkernel::objtab::ObjMap::new`
callee `<[u64; 4] as IntoIterator>::into_iter` in `microkernel::objtab::ObjMap::slot_id::{closure#0}`
callee `<core::array::IntoIter<u64, 4> as Iterator>::next` in `microkernel::objtab::ObjMap::slot_id::{closure#0}`
loop `microkernel::objtab::ObjMap::slot_id::{closure#0}` header bb2
callee `<&u64 as PartialEq>::eq` in `microkernel::objtab::eq`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::objtab::fmt`
callee `Formatter::write_str` in `microkernel::objtab::fmt`
callee `Formatter::debug_struct_field2_finish` in `microkernel::pairrec::fmt`
callee `<Enumerate<core::slice::Iter<'_, Candidate>> as IntoIterator>::into_iter` in `microkernel::partition_next`
callee `<Enumerate<core::slice::Iter<'_, Candidate>> as Iterator>::next` in `microkernel::partition_next`
callee `<core::slice::Iter<'_, Candidate> as Iterator>::enumerate` in `microkernel::partition_next`
callee `Option::or` in `microkernel::partition_next`
loop `microkernel::partition_next` header bb9
callee `GuardedVec::reload` in `microkernel::parts_reseal`
loop `microkernel::parts_reseal` header bb5
callee `alloc::vec::Vec::into_boxed_slice` in `microkernel::partsched::PartSched::new`
callee `alloc::vec::from_elem` in `microkernel::partsched::PartSched::new`
callee `<Enumerate<core::slice::Iter<'_, (Option<u8>, bool, u64)>> as IntoIterator>::into_iter` in `microkernel::priority_next`
callee `<Enumerate<core::slice::Iter<'_, (Option<u8>, bool, u64)>> as Iterator>::next` in `microkernel::priority_next`
callee `<core::slice::Iter<'_, (Option<u8>, bool, u64)> as Iterator>::enumerate` in `microkernel::priority_next`
loop `microkernel::priority_next` header bb4
callee `core::num::leading_zeros` in `microkernel::probes_for`
callee `Formatter::debug_struct_field2_finish` in `microkernel::pure::fmt`
callee `Formatter::debug_struct_field5_finish` in `microkernel::pure::fmt`
callee `<Copied<Filter<core::slice::Iter<'_, (u64, u64)>, {closure@crates\microkernel\src\pvtime.rs:107:63: 107:66}>> as Iterator>::collect` in `microkernel::pvtime::PartVTime::snapshot`
callee `<Filter<core::slice::Iter<'_, (u64, u64)>, {closure@crates\microkernel\src\pvtime.rs:107:63: 107:66}> as Iterator>::copied` in `microkernel::pvtime::PartVTime::snapshot`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::filter` in `microkernel::pvtime::PartVTime::snapshot`
callee `core::slice::sort_unstable` in `microkernel::pvtime::PartVTime::snapshot`
callee `<core::slice::Iter<'_, (u16, u16)> as Iterator>::any` in `microkernel::range_touches_never_grantable`
callee `<u64 as TryFrom<usize>>::try_from` in `microkernel::rd_word`
callee `core::num::from_le_bytes` in `microkernel::rd_word`
callee `<M as EvMem>::read` in `microkernel::read_block`
loop `microkernel::read_block` header bb4
callee `<Result<CptrList, gate::GateReturn> as FromResidual<Result<Infallible, gate::GateReturn>>>::from_residual` in `microkernel::read_cptr_list`
callee `<Result<usize, gate::GateReturn> as Try>::branch` in `microkernel::read_cptr_list`
callee `<[u8; 56] as IndexMut<core::ops::Range<usize>>>::index_mut` in `microkernel::recv_bytes`
callee `core::slice::copy_from_slice` in `microkernel::recv_bytes`
loop `microkernel::recv_bytes` header bb4
callee `<Option<usize> as PartialEq>::eq` in `microkernel::region_untyped_frames_reserve`
loop `microkernel::replace_with_children` header bb19
loop `microkernel::replace_with_children` header bb36
callee `<u16 as TryFrom<u64>>::try_from` in `microkernel::retype_kind`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::retype_kind`
callee `Rights::contains` in `microkernel::ring::Kernel::ring_cap`
callee `Ring::init_header` in `microkernel::ring::Kernel::ring_create_call`
callee `Ring::total_bytes` in `microkernel::ring::Kernel::ring_create_call`
callee `core::num::from_le_bytes` in `microkernel::ring::Kernel::ring_create_call`
loop `microkernel::ring::Kernel::ring_create_call` header bb24
callee `<[(u64, u64); 3] as IntoIterator>::into_iter` in `microkernel::ring::Kernel::ring_enter_call`
callee `<core::array::IntoIter<(u64, u64), 3> as Iterator>::next` in `microkernel::ring::Kernel::ring_enter_call`
callee `Entered::value` in `microkernel::ring::Kernel::ring_enter_call`
callee `RefCell::new` in `microkernel::ring::Kernel::ring_enter_call`
callee `Ring::enter` in `microkernel::ring::Kernel::ring_enter_call`
loop `microkernel::ring::Kernel::ring_enter_call` header bb27
callee `<Option<(u64, Option<u64>, u64, Ring)> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::ring::Kernel::ring_load`
callee `Ring::from_parts` in `microkernel::ring::Kernel::ring_load`
loop `microkernel::ring::Kernel::ring_scrub` header bb3
loop `microkernel::ring::Kernel::ring_state_write` header bb16
loop `microkernel::ring::Kernel::ring_unbind_owner` header bb3
callee `<Enumerate<core::slice::Iter<'_, (u64, u64)>> as IntoIterator>::into_iter` in `microkernel::root_cnode_region`
callee `<Enumerate<core::slice::Iter<'_, (u64, u64)>> as Iterator>::next` in `microkernel::root_cnode_region`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::enumerate` in `microkernel::root_cnode_region`
loop `microkernel::root_cnode_region` header bb4
callee `<Take<core::slice::Iter<'_, RootRec>> as Iterator>::filter_map` in `microkernel::rootidx::RootTable::births`
callee `<core::slice::Iter<'_, RootRec> as Iterator>::take` in `microkernel::rootidx::RootTable::births`
callee `<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}> as Iterator>::map` in `microkernel::rootidx::RootTable::born_roots`
callee `<Take<core::slice::Iter<'_, RootRec>> as Iterator>::filter` in `microkernel::rootidx::RootTable::born_roots`
callee `<core::slice::Iter<'_, RootRec> as Iterator>::take` in `microkernel::rootidx::RootTable::born_roots`
callee `alloc::vec::Vec::into_boxed_slice` in `microkernel::rootidx::RootTable::with_capacity`
callee `alloc::vec::from_elem` in `microkernel::rootidx::RootTable::with_capacity`
callee `Formatter::debug_struct_field4_finish` in `microkernel::rootidx::fmt`
callee `Formatter::debug_struct_field1_finish` in `microkernel::rowseal::fmt`
callee `alloc::collections::BTreeMap::contains_key` in `microkernel::rv::Kernel::rv_object`
callee `alloc::collections::BTreeMap::contains_key` in `microkernel::rv::Kernel::rv_of`
callee `<Keys<'_, u64, vcpu::record::VcpuHome> as IntoIterator>::into_iter` in `microkernel::rv::Kernel::rv_task_has_vcpu`
callee `<Keys<'_, u64, vcpu::record::VcpuHome> as Iterator>::next` in `microkernel::rv::Kernel::rv_task_has_vcpu`
callee `alloc::collections::BTreeMap::keys` in `microkernel::rv::Kernel::rv_task_has_vcpu`
loop `microkernel::rv::Kernel::rv_task_has_vcpu` header bb3
callee `<vcpu::VcpuState as PartialEq>::ne` in `microkernel::rv::Kernel::rv_task_has_vcpu::{closure#1}`
callee `<Take<core::slice::Iter<'_, u64>> as Iterator>::copied` in `microkernel::rv::Kernel::rv_task_retired`
loop `microkernel::rv::Kernel::rv_task_retired` header bb6
callee `<Result<(Option<u64>, Option<u64>), vcpu::HvError> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_bind`
callee `<Result<vcpu::VcpuState, vcpu::HvError> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_bind`
callee `<vcpu::VcpuState as PartialEq>::eq` in `microkernel::rv::Kernel::rvcpu_bind`
callee `<Result<synthyrion_capauth::SealedCap, vcpu::HvError> as FromResidual<Result<Infallible, vcpu::HvError>>>::from_residual` in `microkernel::rv::Kernel::rvcpu_create`
callee `<Option<(Option<u64>, Option<u64>)> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_enter`
callee `<Option<RvEntry> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv::Kernel::rvcpu_enter`
callee `<Option<rv::RvRegs> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_enter`
callee `<Option<rv::RvVsCsrs> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_enter`
callee `<vcpu::VcpuState as PartialEq>::ne` in `microkernel::rv::Kernel::rvcpu_enter`
callee `<Option<rv::RvExit> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv::Kernel::rvcpu_exit_info`
callee `<Option<vcpu::VcpuState> as PartialEq>::ne` in `microkernel::rv::Kernel::rvcpu_exited`
callee `<Result<vcpu::VcpuState, vcpu::HvError> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_inject`
callee `<vcpu::VcpuState as PartialEq>::eq` in `microkernel::rv::Kernel::rvcpu_inject`
callee `<vcpu::VcpuState as PartialEq>::eq` in `microkernel::rv::Kernel::rvcpu_poll`
callee `<Option<rv::RvRegs> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv::Kernel::rvcpu_regs`
callee `<Option<vcpu::VcpuState> as PartialEq>::ne` in `microkernel::rv::Kernel::rvcpu_save_vs`
callee `<Result<vcpu::VcpuState, vcpu::HvError> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_set_regs`
callee `<Result<vcpu::VcpuState, vcpu::HvError> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_set_vs`
callee `<Result<(Option<u64>, Option<u64>), vcpu::HvError> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_start`
callee `<Result<vcpu::VcpuState, vcpu::HvError> as Try>::branch` in `microkernel::rv::Kernel::rvcpu_start`
callee `<vcpu::VcpuState as PartialEq>::ne` in `microkernel::rv::Kernel::rvcpu_start`
callee `<Option<vcpu::VcpuState> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv::Kernel::rvcpu_state`
callee `<Option<rv::RvVsCsrs> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv::Kernel::rvcpu_vs`
callee `<[u64; 32] as Default>::default` in `microkernel::rv::default`
callee `<u64 as Default>::default` in `microkernel::rv::default`
callee `<[u64; 32] as PartialEq>::eq` in `microkernel::rv::eq`
callee `<rv::RvRegs as PartialEq>::eq` in `microkernel::rv::eq`
callee `<rv::RvVsCsrs as PartialEq>::eq` in `microkernel::rv::eq`
callee `Formatter::debug_struct_field2_finish` in `microkernel::rv::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::rv::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::rv::fmt`
callee `Formatter::write_str` in `microkernel::rv::fmt`
callee `alloc::collections::BTreeMap::get` in `microkernel::rv_record::Kernel::rset`
callee `alloc::collections::BTreeMap::remove` in `microkernel::rv_record::Kernel::rv_drop`
callee `<Option<rv::RvExit> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv_record::Kernel::rv_exit_of`
callee `<Zip<core::slice::IterMut<'_, u64>, Keys<'_, u64, vcpu::record::VcpuHome>> as IntoIterator>::into_iter` in `microkernel::rv_record::Kernel::rv_ids`
callee `<Zip<core::slice::IterMut<'_, u64>, Keys<'_, u64, vcpu::record::VcpuHome>> as Iterator>::next` in `microkernel::rv_record::Kernel::rv_ids`
callee `alloc::collections::BTreeMap::keys` in `microkernel::rv_record::Kernel::rv_ids`
loop `microkernel::rv_record::Kernel::rv_ids` header bb5
callee `<rv::RvVsCsrs as Default>::default` in `microkernel::rv_record::Kernel::rv_place`
callee `alloc::collections::BTreeMap::insert` in `microkernel::rv_record::Kernel::rv_place`
loop `microkernel::rv_record::Kernel::rv_put_regs` header bb5
callee `<Option<rv::RvRegs> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv_record::Kernel::rv_regs_of`
loop `microkernel::rv_record::Kernel::rv_regs_of` header bb9
callee `<Option<vcpu::VcpuState> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv_record::Kernel::rv_state`
callee `<Option<vcpu::VcpuState> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv_record::Kernel::rv_state_peek`
callee `<Option<rv::RvVsCsrs> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::rv_record::Kernel::rv_vs_of`
callee `alloc::collections::BTreeMap::get` in `microkernel::rv_record::Kernel::rvcpu_frame`
callee `alloc::collections::BTreeMap::get` in `microkernel::rv_record::Kernel::rw`
callee `alloc::collections::BTreeMap::get` in `microkernel::rv_record::Kernel::rw_peek`
callee `<Filter<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}>, {closure@crates\microkernel\src\selfheal.rs:337:84: 337:87}> as Iterator>::collect` in `microkernel::selfheal::Kernel::contain_poisoned`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::selfheal::Kernel::contain_poisoned`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::selfheal::Kernel::contain_poisoned`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as Iterator>::filter` in `microkernel::selfheal::Kernel::contain_poisoned`
callee `BTreeSet::insert` in `microkernel::selfheal::Kernel::contain_poisoned`
loop `microkernel::selfheal::Kernel::contain_poisoned` header bb5
callee `BTreeSet::contains` in `microkernel::selfheal::Kernel::contain_poisoned::{closure#0}`
callee `AddressSpace::is_poisoned` in `microkernel::selfheal::Kernel::contain_poisoned::{closure#0}::{closure#0}`
callee `<CapReport as Default>::default` in `microkernel::selfheal::Kernel::selfheal_caps`
callee `ScrubStats::add` in `microkernel::selfheal::Kernel::selfheal_caps`
callee `core::mem::take` in `microkernel::selfheal::Kernel::selfheal_caps`
callee `FaultLog::recent` in `microkernel::selfheal::Kernel::selfheal_event`
callee `<CapReport as Default>::default` in `microkernel::selfheal::Kernel::selfheal_objs`
callee `<ObjReport as Default>::default` in `microkernel::selfheal::Kernel::selfheal_objs`
callee `ScrubStats::add` in `microkernel::selfheal::Kernel::selfheal_objs`
callee `core::mem::take` in `microkernel::selfheal::Kernel::selfheal_objs`
callee `AddressSpace::is_poisoned` in `microkernel::selfheal::Kernel::selfheal_space_poisoned::{closure#0}`
callee `<ScrubStats as Default>::default` in `microkernel::selfheal::Kernel::selfheal_step`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as Iterator>::find` in `microkernel::selfheal::Kernel::selfheal_step`
callee `KeyScrubber::advance` in `microkernel::selfheal::Kernel::selfheal_step`
callee `KeyScrubber::key` in `microkernel::selfheal::Kernel::selfheal_step`
callee `KeyScrubber::locate` in `microkernel::selfheal::Kernel::selfheal_step`
callee `ScrubStats::add` in `microkernel::selfheal::Kernel::selfheal_step`
loop `microkernel::selfheal::Kernel::selfheal_step` header bb39
callee `AddressSpace::is_poisoned` in `microkernel::selfheal::Kernel::selfheal_step::{closure#0}::{closure#0}`
callee `AddressSpace::needs_heal` in `microkernel::selfheal::Kernel::selfheal_step::{closure#0}::{closure#0}`
callee `AddressSpace::heal` in `microkernel::selfheal::Kernel::selfheal_step::{closure#1}`
callee `AddressSpace::scrub` in `microkernel::selfheal::Kernel::selfheal_step::{closure#2}`
callee `<ScrubStats as Default>::default` in `microkernel::selfheal::Kernel::selfheal_tasks`
callee `<Take<core::slice::Iter<'_, u64>> as Iterator>::copied` in `microkernel::selfheal::Kernel::selfheal_tasks`
callee `ScrubStats::add` in `microkernel::selfheal::Kernel::selfheal_tasks`
loop `microkernel::selfheal::Kernel::selfheal_tasks` header bb14
callee `<BTreeSet<u64> as Clone>::clone` in `microkernel::selfheal::clone`
callee `<CapCursor as Clone>::clone` in `microkernel::selfheal::clone`
callee `<FaultLog<64> as Clone>::clone` in `microkernel::selfheal::clone`
callee `<KeyScrubber as Clone>::clone` in `microkernel::selfheal::clone`
callee `<ObjCursor as Clone>::clone` in `microkernel::selfheal::clone`
callee `<BTreeSet<u64> as Default>::default` in `microkernel::selfheal::default`
callee `<CapCursor as Default>::default` in `microkernel::selfheal::default`
callee `<FaultLog<64> as Default>::default` in `microkernel::selfheal::default`
callee `<KeyScrubber as Default>::default` in `microkernel::selfheal::default`
callee `<ObjCursor as Default>::default` in `microkernel::selfheal::default`
callee `<bool as Default>::default` in `microkernel::selfheal::default`
callee `<u64 as Default>::default` in `microkernel::selfheal::default`
callee `<usize as Default>::default` in `microkernel::selfheal::default`
callee `Formatter::debug_struct_fields_finish` in `microkernel::selfheal::fmt`
loop `microkernel::selfheal_units` header bb3
loop `microkernel::sharetab::ShareTab::find` header bb2
callee `<Option<Share> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::sharetab::ShareTab::get`
loop `microkernel::sharetab::ShareTab::insert` header bb11
callee `<Option<Share> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::sharetab::ShareTab::remove`
loop `microkernel::sharetab::ShareTab::remove` header bb9
callee `<F as FnMut<(u64, &mut Share)>>::call_mut` in `microkernel::sharetab::ShareTab::retain`
loop `microkernel::sharetab::ShareTab::retain` header bb1
loop `microkernel::sharetab::ShareTab::retain` header bb17
callee `Formatter::write_str` in `microkernel::sharetab::fmt`
callee `<Option<&mut Option<u64>> as Try>::branch` in `microkernel::smpcore::Kernel::block_core`
callee `<alloc::vec::Vec<Option<u64>> as DerefMut>::deref_mut` in `microkernel::smpcore::Kernel::block_core`
callee `<alloc::vec::Vec<bool> as Deref>::deref` in `microkernel::smpcore::Kernel::core_switched_domain`
callee `<alloc::vec::Vec<Option<u64>> as Index<usize>>::index` in `microkernel::smpcore::Kernel::dispatch_core`
callee `<alloc::vec::Vec<Option<u64>> as IndexMut<usize>>::index_mut` in `microkernel::smpcore::Kernel::dispatch_core`
callee `<alloc::vec::Vec<bool> as IndexMut<usize>>::index_mut` in `microkernel::smpcore::Kernel::dispatch_core`
callee `<alloc::vec::Vec<u64> as Index<usize>>::index` in `microkernel::smpcore::Kernel::dispatch_core`
callee `<alloc::vec::Vec<u64> as IndexMut<usize>>::index_mut` in `microkernel::smpcore::Kernel::dispatch_core`
callee `<u64 as TryFrom<usize>>::try_from` in `microkernel::smpcore::Kernel::dispatch_core`
callee `copy _62` in `microkernel::smpcore::Kernel::dispatch_core`
callee `<core::slice::Iter<'_, Option<u64>> as Iterator>::position` in `microkernel::smpcore::Kernel::running_on`
callee `<Skip<core::slice::Iter<'_, Option<u64>>> as Iterator>::any` in `microkernel::smpcore::Kernel::set_core_count`
callee `<core::slice::Iter<'_, Option<u64>> as Iterator>::skip` in `microkernel::smpcore::Kernel::set_core_count`
callee `alloc::vec::Vec::resize` in `microkernel::smpcore::Kernel::set_core_count`
callee `<alloc::vec::Vec<Option<u64>> as DerefMut>::deref_mut` in `microkernel::smpcore::Kernel::smp_forget`
loop `microkernel::smpcore::Kernel::smp_forget` header bb4
callee `<Option<&mut Option<u64>> as Try>::branch` in `microkernel::smpcore::Kernel::yield_core`
callee `<alloc::vec::Vec<Option<u64>> as DerefMut>::deref_mut` in `microkernel::smpcore::Kernel::yield_core`
callee `Box::new_uninit` in `microkernel::smpcore::default`
callee `alloc::boxed::box_assume_init_into_vec_unsafe` in `microkernel::smpcore::default`
callee `core::num::is_multiple_of` in `microkernel::space::Kernel::check_free_range`
callee `AddressSpace::translate` in `microkernel::space::Kernel::check_free_range::{closure#0}`
loop `microkernel::space::Kernel::check_free_range::{closure#0}` header bb2
callee `<Flatten<core::iter::Scan<Enumerate<core::slice::Iter<'_, [u64; 5]>>, usize, {closure@crates\microkernel\src\frametab.rs:371:30: 371:44}>> as IntoIterator>::into_iter` in `microkernel::space::Kernel::frame_holders`
callee `<Flatten<core::iter::Scan<Enumerate<core::slice::Iter<'_, [u64; 5]>>, usize, {closure@crates\microkernel\src\frametab.rs:371:30: 371:44}>> as Iterator>::next` in `microkernel::space::Kernel::frame_holders`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::space::Kernel::frame_holders`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::space::Kernel::frame_holders`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as IntoIterator>::into_iter` in `microkernel::space::Kernel::frame_holders`
callee `<core::iter::Map<Filter<Take<core::slice::Iter<'_, RootRec>>, {closure@crates\microkernel\src\rootidx.rs:174:46: 174:49}>, {closure@crates\microkernel\src\rootidx.rs:174:73: 174:76}> as Iterator>::next` in `microkernel::space::Kernel::frame_holders`
callee `AddressSpace::leaf_frames` in `microkernel::space::Kernel::frame_holders`
callee `BTreeSet::insert` in `microkernel::space::Kernel::frame_holders`
callee `ReadHealth::merge` in `microkernel::space::Kernel::frame_holders`
callee `RoView::health` in `microkernel::space::Kernel::frame_holders`
callee `alloc::collections::BTreeMap::entry` in `microkernel::space::Kernel::frame_holders`
callee `alloc::collections::btree_map::Entry::or_default` in `microkernel::space::Kernel::frame_holders`
loop `microkernel::space::Kernel::frame_holders` header bb4
loop `microkernel::space::Kernel::frame_holders` header bb25
callee `alloc::collections::BTreeMap::get` in `microkernel::space::Kernel::frame_word`
callee `FrameAllocator::used` in `microkernel::space::Kernel::frames_in_use`
callee `FrameAllocator::total` in `microkernel::space::Kernel::frames_total`
callee `<Values<'_, (u64, u64), FilePage> as Iterator>::any` in `microkernel::space::Kernel::is_file_frame::{closure#0}`
callee `alloc::collections::BTreeMap::values` in `microkernel::space::Kernel::is_file_frame::{closure#0}`
callee `<Enumerate<alloc::vec::IntoIter<u64>> as IntoIterator>::into_iter` in `microkernel::space::Kernel::map_anonymous`
callee `<Enumerate<alloc::vec::IntoIter<u64>> as Iterator>::next` in `microkernel::space::Kernel::map_anonymous`
callee `<alloc::vec::IntoIter<u64> as Iterator>::enumerate` in `microkernel::space::Kernel::map_anonymous`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::space::Kernel::map_anonymous`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::space::Kernel::map_anonymous`
callee `alloc::vec::Vec::push` in `microkernel::space::Kernel::map_anonymous`
callee `alloc::vec::Vec::with_capacity` in `microkernel::space::Kernel::map_anonymous`
loop `microkernel::space::Kernel::map_anonymous` header bb10
loop `microkernel::space::Kernel::map_anonymous` header bb18
loop `microkernel::space::Kernel::map_anonymous` header bb27
loop `microkernel::space::Kernel::map_anonymous` header bb36
loop `microkernel::space::Kernel::map_anonymous` header bb44
callee `AddressSpace::map` in `microkernel::space::Kernel::map_anonymous::{closure#0}`
callee `<Enumerate<alloc::vec::IntoIter<u64>> as IntoIterator>::into_iter` in `microkernel::space::Kernel::map_file`
callee `<Enumerate<alloc::vec::IntoIter<u64>> as Iterator>::next` in `microkernel::space::Kernel::map_file`
callee `<alloc::vec::IntoIter<(u64, u64)> as Iterator>::next` in `microkernel::space::Kernel::map_file`
callee `<alloc::vec::IntoIter<u64> as Iterator>::enumerate` in `microkernel::space::Kernel::map_file`
callee `<alloc::vec::Vec<(u64, u64)> as IntoIterator>::into_iter` in `microkernel::space::Kernel::map_file`
callee `<alloc::vec::Vec<u64> as Clone>::clone` in `microkernel::space::Kernel::map_file`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::space::Kernel::map_file`
callee `<dyn filedoor::FileBacking as filedoor::FileBacking>::read` in `microkernel::space::Kernel::map_file`
callee `alloc::collections::BTreeMap::get` in `microkernel::space::Kernel::map_file`
callee `alloc::collections::BTreeMap::insert` in `microkernel::space::Kernel::map_file`
callee `alloc::collections::BTreeMap::remove` in `microkernel::space::Kernel::map_file`
callee `alloc::vec::Vec::push` in `microkernel::space::Kernel::map_file`
callee `alloc::vec::Vec::with_capacity` in `microkernel::space::Kernel::map_file`
callee `core::num::div_ceil` in `microkernel::space::Kernel::map_file`
callee `core::num::from_le_bytes` in `microkernel::space::Kernel::map_file`
callee `core::slice::contains` in `microkernel::space::Kernel::map_file`
loop `microkernel::space::Kernel::map_file` header bb44
loop `microkernel::space::Kernel::map_file` header bb88
loop `microkernel::space::Kernel::map_file` header bb95
loop `microkernel::space::Kernel::map_file` header bb107
loop `microkernel::space::Kernel::map_file` header bb118
callee `AddressSpace::leaf_frames` in `microkernel::space::Kernel::map_file::{closure#0}`
callee `AddressSpace::leaf_frames` in `microkernel::space::Kernel::map_file::{closure#1}`
callee `AddressSpace::map` in `microkernel::space::Kernel::map_file::{closure#3}`
callee `<FilterMap<core::ops::Range<usize>, {closure@crates\microkernel\src\space.rs:1506:43: 1506:46}> as Iterator>::map` in `microkernel::space::Kernel::open_shares`
callee `<core::iter::Map<FilterMap<core::ops::Range<usize>, {closure@crates\microkernel\src\space.rs:1506:43: 1506:46}>, {closure@crates\microkernel\src\space.rs:1506:93: 1506:102}> as Iterator>::collect` in `microkernel::space::Kernel::open_shares`
callee `<core::ops::Range<usize> as Iterator>::filter_map` in `microkernel::space::Kernel::open_shares`
callee `alloc::collections::BTreeMap::get` in `microkernel::space::Kernel::physical_frame_word`
callee `<&alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::space::Kernel::reap_tasks_of`
callee `<Filter<alloc::collections::btree_map::Iter<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\space.rs:930:21: 930:29}> as Iterator>::map` in `microkernel::space::Kernel::reap_tasks_of`
callee `<alloc::collections::btree_map::Iter<'_, u64, tasktable::Task> as Iterator>::filter` in `microkernel::space::Kernel::reap_tasks_of`
callee `<core::iter::Map<Filter<alloc::collections::btree_map::Iter<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\space.rs:930:21: 930:29}>, {closure@crates\microkernel\src\space.rs:931:18: 931:28}> as Iterator>::collect` in `microkernel::space::Kernel::reap_tasks_of`
loop `microkernel::space::Kernel::reap_tasks_of` header bb9
callee `<alloc::collections::btree_map::Iter<'_, (u64, u64), FilePage> as Iterator>::find` in `microkernel::space::Kernel::release_frame_kernel`
callee `<dyn filedoor::FileBacking as filedoor::FileBacking>::cap_for` in `microkernel::space::Kernel::release_frame_kernel`
callee `<dyn filedoor::FileBacking as filedoor::FileBacking>::write` in `microkernel::space::Kernel::release_frame_kernel`
callee `alloc::collections::BTreeMap::get` in `microkernel::space::Kernel::release_frame_kernel`
callee `alloc::collections::BTreeMap::iter` in `microkernel::space::Kernel::release_frame_kernel`
callee `alloc::collections::BTreeMap::remove` in `microkernel::space::Kernel::release_frame_kernel`
callee `<Enumerate<alloc::vec::IntoIter<u64>> as IntoIterator>::into_iter` in `microkernel::space::Kernel::share_attach`
callee `<Enumerate<alloc::vec::IntoIter<u64>> as Iterator>::next` in `microkernel::space::Kernel::share_attach`
callee `<alloc::vec::IntoIter<u64> as Iterator>::enumerate` in `microkernel::space::Kernel::share_attach`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::space::Kernel::share_attach`
callee `<core::slice::Iter<'_, u64> as Iterator>::any` in `microkernel::space::Kernel::share_attach`
callee `AddressSpace::translate` in `microkernel::space::Kernel::share_attach`
callee `alloc::vec::Vec::push` in `microkernel::space::Kernel::share_attach`
callee `alloc::vec::Vec::with_capacity` in `microkernel::space::Kernel::share_attach`
loop `microkernel::space::Kernel::share_attach` header bb22
loop `microkernel::space::Kernel::share_attach` header bb63
callee `AddressSpace::leaf_frames` in `microkernel::space::Kernel::share_attach::{closure#0}`
callee `core::slice::contains` in `microkernel::space::Kernel::share_attach::{closure#1}`
callee `AddressSpace::map` in `microkernel::space::Kernel::share_attach::{closure#4}`
callee `<ValuesMut<'_, (u64, u64), FilePage> as Iterator>::find` in `microkernel::space::Kernel::write_frame_word`
callee `alloc::collections::BTreeMap::insert` in `microkernel::space::Kernel::write_frame_word`
callee `alloc::collections::BTreeMap::values_mut` in `microkernel::space::Kernel::write_frame_word`
callee `synthyrion_vmm::treeview::attach_new` in `microkernel::space::Spaces::birth`
callee `synthyrion_vmm::treeview::commit` in `microkernel::space::Spaces::birth`
callee `synthyrion_vmm::treeview::attach` in `microkernel::space::Spaces::edit`
callee `synthyrion_vmm::treeview::commit` in `microkernel::space::Spaces::edit`
callee `<KView<'_> as synthyrion_vmm::TableMem>::provide` in `microkernel::space::Spaces::legacy_window_prepare`
callee `record_table_side` in `microkernel::space::Spaces::legacy_window_prepare`
callee `tables_needed_range_in_empty_tree` in `microkernel::space::Spaces::legacy_window_prepare`
callee `AddressSpace::table_supply_len` in `microkernel::space::Spaces::legacy_window_prepare::{closure#0}`
callee `AddressSpace::tables_needed_range` in `microkernel::space::Spaces::legacy_window_prepare::{closure#0}`
callee `AddressSpace::tables_needed_range` in `microkernel::space::Spaces::legacy_window_prepare::{closure#1}`
callee `AddressSpace::table_supply_len` in `microkernel::space::Spaces::legacy_window_prepare::{closure#2}`
callee `AddressSpace::provide_table_frame` in `microkernel::space::Spaces::legacy_window_prepare::{closure#3}`
callee `AddressSpace::add_table_frames` in `microkernel::space::Spaces::legacy_window_prepare::{closure#4}`
callee `copy _3` in `microkernel::space::Spaces::make_window`
callee `<core::iter::Map<core::slice::Iter<'_, (u64, u64, u64)>, {closure@crates\microkernel\src\space.rs:535:28: 535:52}> as Iterator>::collect` in `microkernel::space::Spaces::map_of`
callee `<core::slice::Iter<'_, (u64, u64, u64)> as Iterator>::map` in `microkernel::space::Spaces::map_of`
callee `<FrameHold as Default>::default` in `microkernel::space::Spaces::new_empty_arena`
callee `<TableWin as Default>::default` in `microkernel::space::Spaces::new_empty_arena`
callee `Box::new` in `microkernel::space::Spaces::new_empty_arena`
callee `FrameAllocator::new` in `microkernel::space::Spaces::new_empty_arena`
callee `<KWindow as synthyrion_vmm::TableMem>::can_provide` in `microkernel::space::Spaces::probe_covers`
callee `alloc::collections::BTreeMap::remove` in `microkernel::space::Spaces::return_pool`
loop `microkernel::space::Spaces::return_pool` header bb1
callee `RoView::attach` in `microkernel::space::Spaces::space`
callee `<KView<'_> as synthyrion_vmm::TableMem>::provide` in `microkernel::space::Spaces::stage_root`
callee `<FrameHold as Default>::default` in `microkernel::space::Spaces::with_frames_reserve`
callee `<TableWin as Default>::default` in `microkernel::space::Spaces::with_frames_reserve`
callee `Box::new` in `microkernel::space::Spaces::with_frames_reserve`
callee `FrameAllocator::new` in `microkernel::space::Spaces::with_frames_reserve`
callee `<&[(u64, u64)] as IntoIterator>::into_iter` in `microkernel::space::Spaces::with_memory`
callee `<core::iter::Map<core::slice::Iter<'_, (u64, u64)>, {closure@crates\microkernel\src\space.rs:323:62: 323:65}> as Iterator>::sum` in `microkernel::space::Spaces::with_memory`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::map` in `microkernel::space::Spaces::with_memory`
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::next` in `microkernel::space::Spaces::with_memory`
callee `alloc::slice::to_vec` in `microkernel::space::Spaces::with_memory`
callee `alloc::vec::Vec::push` in `microkernel::space::Spaces::with_memory`
loop `microkernel::space::Spaces::with_memory` header bb7
callee `<KWindow as Default>::default` in `microkernel::space::default`
callee `<Option<fn(u64) -> Box<dyn synthyrion_vmm::TableMem>> as Default>::default` in `microkernel::space::default`
callee `<u64 as Default>::default` in `microkernel::space::default`
callee `<usize as Default>::default` in `microkernel::space::default`
callee `Formatter::debug_struct_fields_finish` in `microkernel::space::fmt`
callee `Formatter::write_str` in `microkernel::space::fmt`
callee `<KWindow as synthyrion_vmm::TableMem>::can_provide` in `microkernel::spaceview::can_provide::{closure#0}`
callee `Box::new` in `microkernel::spaceview::clone_box`
callee `<Col<'_> as Default>::default` in `microkernel::spaceview::default`
callee `<Win<'_> as Default>::default` in `microkernel::spaceview::default`
callee `<KWindow as synthyrion_vmm::TableMem>::entry_map` in `microkernel::spaceview::entry_map`
callee `<KWindow as synthyrion_vmm::TableMem>::frames` in `microkernel::spaceview::frames::{closure#0}`
callee `<KWindow as synthyrion_vmm::TableMem>::provide` in `microkernel::spaceview::provide`
callee `<KWindow as synthyrion_vmm::TableMem>::pair` in `microkernel::spaceview::tables`
callee `<Option<&FrameTab> as Try>::branch` in `microkernel::spaceview::tables`
callee `<Option<&KWindow> as Try>::branch` in `microkernel::spaceview::tables`
callee `<Option<(&[u64], &[u32])> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::spaceview::tables`
callee `<KWindow as synthyrion_vmm::TableMem>::pair_mut` in `microkernel::spaceview::tables_mut`
callee `<Option<&FrameTab> as Try>::branch` in `microkernel::spaceview::tables_mut`
callee `<Option<&mut KWindow> as Try>::branch` in `microkernel::spaceview::tables_mut`
callee `<Option<(&mut [u64], &mut [u32])> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::spaceview::tables_mut`
callee `<KWindow as synthyrion_vmm::TableMem>::pair_mut_map` in `microkernel::spaceview::tables_mut_map`
callee `<Option<&FrameTab> as Try>::branch` in `microkernel::spaceview::tables_mut_map`
callee `<Option<&mut KWindow> as Try>::branch` in `microkernel::spaceview::tables_mut_map`
callee `<Option<(&mut [u64], &mut [u32], &[(u64, u64, u64)])> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::spaceview::tables_mut_map`
callee `<KWindow as synthyrion_vmm::TableMem>::unprovide` in `microkernel::spaceview::unprovide`
callee `<Enumerate<core::slice::Iter<'_, (u64, u64, bool)>> as IntoIterator>::into_iter` in `microkernel::stride_next`
callee `<Enumerate<core::slice::Iter<'_, (u64, u64, bool)>> as Iterator>::next` in `microkernel::stride_next`
callee `<core::slice::Iter<'_, (u64, u64, bool)> as Iterator>::enumerate` in `microkernel::stride_next`
loop `microkernel::stride_next` header bb4
callee `<core::slice::Iter<'_, (u64, u64)> as Iterator>::position` in `microkernel::table_region`
loop `microkernel::tablemem::KWindow::real` header bb2
callee `alloc::vec::Vec::into_boxed_slice` in `microkernel::tablemem::RegionMap::new`
callee `alloc::vec::from_elem` in `microkernel::tablemem::RegionMap::new`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::can_provide` in `microkernel::tablemem::can_provide`
callee `<Box<[(u64, u64, u64)]> as Clone>::clone` in `microkernel::tablemem::clone`
callee `<RegionMap as Clone>::clone` in `microkernel::tablemem::clone`
callee `<alloc::vec::Vec<(u64, u64, u64)> as Clone>::clone` in `microkernel::tablemem::clone`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::clone_box` in `microkernel::tablemem::clone`
callee `<KWindow as Clone>::clone` in `microkernel::tablemem::clone_box`
callee `Box::new` in `microkernel::tablemem::clone_box`
callee `Box::new` in `microkernel::tablemem::default`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::frames` in `microkernel::tablemem::fmt`
callee `Arguments::new` in `microkernel::tablemem::fmt`
callee `Formatter::debug_struct_field2_finish` in `microkernel::tablemem::fmt`
callee `Formatter::write_fmt` in `microkernel::tablemem::fmt`
callee `Formatter::write_str` in `microkernel::tablemem::fmt`
callee `core::fmt::rt::Argument::new_display` in `microkernel::tablemem::fmt`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::frames` in `microkernel::tablemem::frames`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::pair` in `microkernel::tablemem::pair`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::pair_mut` in `microkernel::tablemem::pair_mut`
callee `<Option<(&mut [u64], &mut [u32])> as Try>::branch` in `microkernel::tablemem::pair_mut_map`
callee `<Option<(&mut [u64], &mut [u32], &[(u64, u64, u64)])> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::tablemem::pair_mut_map`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::pair_mut` in `microkernel::tablemem::pair_mut_map`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::provide` in `microkernel::tablemem::provide`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::tables` in `microkernel::tablemem::tables`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::tables_mut` in `microkernel::tablemem::tables_mut`
callee `<Option<(&mut [u64], &mut [u32])> as Try>::branch` in `microkernel::tablemem::tables_mut_map`
callee `<Option<(&mut [u64], &mut [u32], &[(u64, u64, u64)])> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::tablemem::tables_mut_map`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::tables_mut` in `microkernel::tablemem::tables_mut_map`
callee `<dyn synthyrion_vmm::TableMem as synthyrion_vmm::TableMem>::unprovide` in `microkernel::tablemem::unprovide`
callee `core::num::div_ceil` in `microkernel::task_frames`
callee `<FrameScrub as Default>::default` in `microkernel::taskframe::TaskFrame::scrub`
callee `<Option<[u64; 18]> as PartialEq>::ne` in `microkernel::taskframe::TaskFrame::scrub`
callee `<Option<[u64; 29]> as PartialEq>::ne` in `microkernel::taskframe::TaskFrame::scrub`
callee `<Option<[u64; 51]> as PartialEq>::ne` in `microkernel::taskframe::TaskFrame::scrub`
callee `<Option<[u64; 63]> as PartialEq>::ne` in `microkernel::taskframe::TaskFrame::scrub`
callee `<Option<[u64; 59]> as PartialEq>::ne` in `microkernel::taskframe::TaskFrame::scrub_grants`
callee `<u64 as Default>::default` in `microkernel::taskframe::default`
callee `<&T as PartialEq>::eq` in `microkernel::taskframe::eq`
callee `Formatter::debug_struct_field3_finish` in `microkernel::taskframe::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::taskframe::fmt`
callee `Formatter::write_str` in `microkernel::taskframe::fmt`
callee `<Option<RtParams> as PartialEq>::eq` in `microkernel::taskrow::eq`
callee `<Option<u8> as PartialEq>::eq` in `microkernel::taskrow::eq`
callee `Formatter::debug_struct_fields_finish` in `microkernel::taskrow::fmt`
callee `<ObjRam as HdrMem>::release` in `microkernel::tasktable::Task::bind_new`
loop `microkernel::tasktable::Task::bind_new` header bb3
loop `microkernel::tasktable::Task::bind_new` header bb16
loop `microkernel::tasktable::Task::bind_new` header bb37
callee `<ObjRam as HdrMem>::claim` in `microkernel::tasktable::Task::bind_new::{closure#0}`
callee `alloc::vec::Vec::push` in `microkernel::tasktable::Task::inbox_drain_all`
loop `microkernel::tasktable::Task::inbox_drain_all` header bb4
callee `<dyn for<'a> FnMut(&'a Delivery) -> bool as FnMut<(&Delivery,)>>::call_mut` in `microkernel::tasktable::Task::inbox_retain`
loop `microkernel::tasktable::Task::inbox_retain` header bb5
callee `<ObjRam as HdrMem>::release` in `microkernel::tasktable::Task::retire_frame`
loop `microkernel::tasktable::Task::retire_frame` header bb6
loop `microkernel::tasktable::Task::retire_frame` header bb15
callee `<Filter<Values<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\tasktable.rs:808:45: 808:48}> as Iterator>::count` in `microkernel::tasktable::TaskTable::homes`
callee `<Filter<Values<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\tasktable.rs:810:43: 810:46}> as Iterator>::count` in `microkernel::tasktable::TaskTable::homes`
callee `<Values<'_, u64, tasktable::Task> as Iterator>::filter` in `microkernel::tasktable::TaskTable::homes`
callee `<Zip<core::iter::Map<alloc::collections::btree_map::Range<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\tasktable.rs:799:52: 799:60}>, core::slice::IterMut<'_, u64>> as IntoIterator>::into_iter` in `microkernel::tasktable::TaskTable::ids_from`
callee `<Zip<core::iter::Map<alloc::collections::btree_map::Range<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\tasktable.rs:799:52: 799:60}>, core::slice::IterMut<'_, u64>> as Iterator>::next` in `microkernel::tasktable::TaskTable::ids_from`
callee `<alloc::collections::btree_map::Range<'_, u64, tasktable::Task> as Iterator>::map` in `microkernel::tasktable::TaskTable::ids_from`
callee `<core::iter::Map<alloc::collections::btree_map::Range<'_, u64, tasktable::Task>, {closure@crates\microkernel\src\tasktable.rs:799:52: 799:60}> as Iterator>::zip` in `microkernel::tasktable::TaskTable::ids_from`
loop `microkernel::tasktable::TaskTable::ids_from` header bb6
callee `alloc::collections::BTreeMap::insert` in `microkernel::tasktable::TaskTable::insert`
callee `<Home as Clone>::clone` in `microkernel::tasktable::clone`
callee `<alloc::collections::BTreeMap<u64, tasktable::Task> as Clone>::clone` in `microkernel::tasktable::clone`
callee `<&u64 as PartialEq>::eq` in `microkernel::tasktable::eq`
callee `Formatter::debug_struct_field1_finish` in `microkernel::tasktable::fmt`
callee `Formatter::debug_struct_field3_finish` in `microkernel::tasktable::fmt`
callee `Formatter::write_str` in `microkernel::tasktable::fmt`
callee `Formatter::debug_struct_fields_finish` in `microkernel::taskxcall::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::taskxgate::fmt`
callee `<[Option<(u64, synthyrion_capauth::SealedCap)>; 4] as Default>::default` in `microkernel::taskxgrants::default`
callee `Formatter::debug_struct_field1_finish` in `microkernel::taskxgrants::fmt`
callee `<Option<(bool, bool)> as PartialEq>::eq` in `microkernel::taskxrow::eq`
callee `<Option<[u64; 18]> as PartialEq>::eq` in `microkernel::taskxrow::eq`
callee `<Option<ipcx::Fault> as PartialEq>::eq` in `microkernel::taskxrow::eq`
callee `Formatter::debug_struct_fields_finish` in `microkernel::taskxrow::fmt`
callee `Rights::contains` in `microkernel::tcb::Kernel::cs_tcb_configure`
callee `<Result<[u64; 18], synthyrion_abi::SysResult> as FromResidual<Result<Infallible, synthyrion_abi::SysResult>>>::from_residual` in `microkernel::tcb::Kernel::cs_tcb_read_registers`
callee `<Result<u64, synthyrion_abi::SysResult> as Try>::branch` in `microkernel::tcb::Kernel::cs_tcb_read_registers`
callee `<Result<(synthyrion_caps::ObjKind, u64, synthyrion_caps::Rights, u64), synthyrion_abi::SysResult> as Try>::branch` in `microkernel::tcb::Kernel::cs_tcb_target`
callee `<Result<u64, synthyrion_abi::SysResult> as FromResidual<Result<Infallible, synthyrion_abi::SysResult>>>::from_residual` in `microkernel::tcb::Kernel::cs_tcb_target`
callee `Rights::contains` in `microkernel::tcb::Kernel::cs_tcb_target`
callee `<Result<&tasktable::Task, synthyrion_abi::SysResult> as Try>::branch` in `microkernel::tcb::Kernel::read_registers_id`
callee `<Result<[u64; 18], synthyrion_abi::SysResult> as FromResidual<Result<Infallible, synthyrion_abi::SysResult>>>::from_residual` in `microkernel::tcb::Kernel::read_registers_id`
callee `<Result<[u64; 18], synthyrion_abi::SysResult> as FromResidual<Result<Infallible, synthyrion_abi::SysResult>>>::from_residual` in `microkernel::tcb::Kernel::tcb_read_registers`
callee `<Result<u64, synthyrion_abi::SysResult> as Try>::branch` in `microkernel::tcb::Kernel::tcb_read_registers`
callee `<Option<&synthyrion_capauth::SealedCap> as Default>::default` in `microkernel::tcb::default`
callee `<Option<u64> as Default>::default` in `microkernel::tcb::default`
callee `<Option<u8> as Default>::default` in `microkernel::tcb::default`
callee `Formatter::debug_struct_field4_finish` in `microkernel::tcb::fmt`
callee `<ChunksExactMut<'_, u8> as Iterator>::zip` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<Zip<ChunksExactMut<'_, u8>, core::slice::Iter<'_, u64>> as IntoIterator>::into_iter` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<Zip<ChunksExactMut<'_, u8>, core::slice::Iter<'_, u64>> as Iterator>::next` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<Zip<core::slice::IterMut<'_, u64>, core::iter::FromFn<{closure@crates\microkernel\src\gate\tcb_rows.rs:36:25: 36:32}>> as IntoIterator>::into_iter` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<Zip<core::slice::IterMut<'_, u64>, core::iter::FromFn<{closure@crates\microkernel\src\gate\tcb_rows.rs:36:25: 36:32}>> as Iterator>::next` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<Zip<core::slice::IterMut<'_, u8>, core::slice::Iter<'_, u8>> as IntoIterator>::into_iter` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<Zip<core::slice::IterMut<'_, u8>, core::slice::Iter<'_, u8>> as Iterator>::next` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<core::iter::FromFn<{closure@crates\microkernel\src\gate\tcb_rows.rs:36:25: 36:32}> as Iterator>::next` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<core::slice::IterMut<'_, u8> as Iterator>::zip` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<u64 as TryFrom<usize>>::try_from` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `and_then (2 closure position(s) seen but 1 closure body(ies) owned: the ordinal is not resolvable)` in `microkernel::tcb_rows::Kernel::serve_tcb`
callee `core::slice::chunks_exact_mut` in `microkernel::tcb_rows::Kernel::serve_tcb`
loop `microkernel::tcb_rows::Kernel::serve_tcb` header bb43
loop `microkernel::tcb_rows::Kernel::serve_tcb` header bb56
loop `microkernel::tcb_rows::Kernel::serve_tcb` header bb112
loop `microkernel::tcb_rows::Kernel::serve_tcb` header bb201
callee `<u8 as TryFrom<u64>>::try_from` in `microkernel::tcb_rows::Kernel::tcb_slot`
callee `Rights::contains` in `microkernel::tcb_rows::Kernel::tcb_slot`
callee `Rights::contains` in `microkernel::tcb_vspace::Kernel::cs_tcb_bind_vspace`
callee `<u128 as From<u64>>::from` in `microkernel::tick_quanta_due`
callee `<u64 as TryFrom<u128>>::try_from` in `microkernel::tick_quanta_due`
callee `<alloc::vec::IntoIter<u64> as Iterator>::next` in `microkernel::timeprot::Kernel::donate_colour_frames`
callee `<alloc::vec::Vec<u64> as IntoIterator>::into_iter` in `microkernel::timeprot::Kernel::donate_colour_frames`
callee `ColourPolicy::mask_of` in `microkernel::timeprot::Kernel::donate_colour_frames`
callee `alloc::vec::Vec::push` in `microkernel::timeprot::Kernel::donate_colour_frames`
callee `alloc::vec::Vec::with_capacity` in `microkernel::timeprot::Kernel::donate_colour_frames`
callee `core::slice::contains` in `microkernel::timeprot::Kernel::donate_colour_frames`
loop `microkernel::timeprot::Kernel::donate_colour_frames` header bb19
loop `microkernel::timeprot::Kernel::donate_colour_frames` header bb34
loop `microkernel::timeprot::Kernel::donate_colour_frames` header bb41
callee `alloc::collections::BTreeMap::insert` in `microkernel::timeprot::Kernel::pin_domain_to_cores`
callee `alloc::collections::BTreeMap::get` in `microkernel::timeprot::Kernel::placement_refused`
loop `microkernel::timeprot::Kernel::placement_refused` header bb13
callee `AddressSpace::table_supply_len` in `microkernel::timeprot::Kernel::reserve_tables_with`
callee `AddressSpace::tables_needed` in `microkernel::timeprot::Kernel::reserve_tables_with`
callee `AddressSpace::tables_needed_range` in `microkernel::timeprot::Kernel::reserve_tables_with`
callee `tables_needed_range_in_empty_tree` in `microkernel::timeprot::Kernel::reserve_tables_with`
callee `AddressSpace::add_table_frames` in `microkernel::timeprot::Kernel::reserve_tables_with::{closure#1}`
callee `ColourPolicy::new` in `microkernel::timeprot::Kernel::set_colour_bits`
callee `ColourPolicy::assign` in `microkernel::timeprot::Kernel::set_domain_colours`
callee `SmtTopology::core_of` in `microkernel::timeprot::Kernel::smt_siblings`
callee `AddressSpace::supplied_table_frames` in `microkernel::timeprot::Kernel::space_table_frames::{closure#0}`
callee `<Option<ColourPolicy> as Default>::default` in `microkernel::timeprot::default`
callee `<Option<SmtTopology> as Default>::default` in `microkernel::timeprot::default`
callee `<alloc::collections::BTreeMap<u64, u64> as Default>::default` in `microkernel::timeprot::default`
callee `<alloc::vec::Vec<u32> as Default>::default` in `microkernel::timeprot::default`
callee `<u64 as Default>::default` in `microkernel::timeprot::default`
callee `<alloc::vec::Vec<CoreTlb> as Deref>::deref` in `microkernel::tlbcores::CoreTlbs::loaded_pcids`
loop `microkernel::tlbcores::CoreTlbs::loaded_pcids` header bb5
callee `alloc::collections::BTreeMap::insert` in `microkernel::tlbcores::Kernel::model_store_through_stale`
callee `alloc::collections::BTreeMap::get` in `microkernel::tlbcores::Kernel::model_word_at`
callee `<CoreTlb as Default>::default` in `microkernel::tlbcores::Kernel::smp_tlb_enable`
callee `alloc::vec::Vec::push` in `microkernel::tlbcores::Kernel::smp_tlb_enable`
callee `alloc::vec::Vec::reserve_exact` in `microkernel::tlbcores::Kernel::smp_tlb_enable`
loop `microkernel::tlbcores::Kernel::smp_tlb_enable` header bb8
callee `<Enumerate<core::slice::IterMut<'_, CoreTlb>> as IntoIterator>::into_iter` in `microkernel::tlbcores::Kernel::space_switch_on`
callee `<Enumerate<core::slice::IterMut<'_, CoreTlb>> as Iterator>::next` in `microkernel::tlbcores::Kernel::space_switch_on`
callee `<Result<&mut CoreTlb, objects::ObjError> as Try>::branch` in `microkernel::tlbcores::Kernel::space_switch_on`
callee `<Result<SpaceSwitch, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::tlbcores::Kernel::space_switch_on`
callee `<Result<SpaceSwitch, objects::ObjError> as Try>::branch` in `microkernel::tlbcores::Kernel::space_switch_on`
callee `<SwitchKind as PartialEq>::eq` in `microkernel::tlbcores::Kernel::space_switch_on`
callee `<alloc::vec::Vec<CoreTlb> as DerefMut>::deref_mut` in `microkernel::tlbcores::Kernel::space_switch_on`
callee `<core::slice::IterMut<'_, CoreTlb> as Iterator>::enumerate` in `microkernel::tlbcores::Kernel::space_switch_on`
loop `microkernel::tlbcores::Kernel::space_switch_on` header bb23
callee `<Result<ShootPlan, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::tlbcores::Kernel::tlb_shootdown_begin`
callee `<Take<core::slice::Iter<'_, TlbInval>> as Iterator>::any` in `microkernel::tlbcores::Kernel::tlb_shootdown_begin`
callee `<alloc::vec::Vec<CoreTlb> as Deref>::deref` in `microkernel::tlbcores::Kernel::tlb_shootdown_begin`
callee `<core::slice::Iter<'_, TlbInval> as Iterator>::take` in `microkernel::tlbcores::Kernel::tlb_shootdown_begin`
loop `microkernel::tlbcores::Kernel::tlb_shootdown_begin` header bb22
callee `<CoreTlb as Default>::default` in `microkernel::tlbcores::Kernel::tlb_shootdown_complete`
callee `<Enumerate<core::slice::IterMut<'_, CoreTlb>> as IntoIterator>::into_iter` in `microkernel::tlbcores::Kernel::tlb_shootdown_complete`
callee `<Enumerate<core::slice::IterMut<'_, CoreTlb>> as Iterator>::next` in `microkernel::tlbcores::Kernel::tlb_shootdown_complete`
callee `<alloc::vec::Vec<CoreTlb> as DerefMut>::deref_mut` in `microkernel::tlbcores::Kernel::tlb_shootdown_complete`
callee `<core::slice::IterMut<'_, CoreTlb> as Iterator>::enumerate` in `microkernel::tlbcores::Kernel::tlb_shootdown_complete`
loop `microkernel::tlbcores::Kernel::tlb_shootdown_complete` header bb8
callee `<core::slice::Iter<'_, u64> as Iterator>::any` in `microkernel::tlbcores::PcidSet::any`
callee `<&u64 as BitAnd<u64>>::bitand` in `microkernel::tlbcores::PcidSet::has::{closure#0}::{closure#0}`
callee `<Option<(u64, u16)> as Clone>::clone` in `microkernel::tlbcores::clone`
callee `<Option<OpenShoot> as Clone>::clone` in `microkernel::tlbcores::clone`
callee `<Option<u64> as Clone>::clone` in `microkernel::tlbcores::clone`
callee `<PcidSet as Clone>::clone` in `microkernel::tlbcores::clone`
callee `<alloc::vec::Vec<CoreTlb> as Clone>::clone` in `microkernel::tlbcores::clone`
callee `alloc::collections::BTreeMap::remove` in `microkernel::tlbcores::crate::space::Spaces::release_held`
loop `microkernel::tlbcores::crate::space::Spaces::release_held` header bb1
callee `<Option<(u64, u16)> as Default>::default` in `microkernel::tlbcores::default`
callee `<Option<OpenShoot> as Default>::default` in `microkernel::tlbcores::default`
callee `<Option<u64> as Default>::default` in `microkernel::tlbcores::default`
callee `<PcidSet as Default>::default` in `microkernel::tlbcores::default`
callee `<alloc::vec::Vec<CoreTlb> as Default>::default` in `microkernel::tlbcores::default`
callee `<bool as Default>::default` in `microkernel::tlbcores::default`
callee `Formatter::debug_struct_field1_finish` in `microkernel::tlbcores::fmt`
callee `Formatter::debug_struct_field2_finish` in `microkernel::tlbcores::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::tlbcores::fmt`
callee `Formatter::debug_struct_field5_finish` in `microkernel::tlbcores::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::tlbcores::fmt`
callee `TicketLock::lock` in `microkernel::userwalk::WindowLock::enter`
callee `TicketLock::new` in `microkernel::userwalk::WindowLock::new`
callee `TicketLock::try_lock` in `microkernel::userwalk::WindowLock::try_enter`
callee `TicketLock::unlock` in `microkernel::userwalk::drop`
callee `alloc::vec::Vec::as_slice` in `microkernel::vaultevid::EvidenceRecord::put`
callee `array::as_slice` in `microkernel::vaultevid::EvidenceRecord::put`
callee `core::num::div_ceil` in `microkernel::vaultevid::EvidenceRecord::put`
callee `core::num::from_le_bytes` in `microkernel::vaultevid::EvidenceRecord::put`
loop `microkernel::vaultevid::EvidenceRecord::put` header bb39
callee `<Option<&[u8]> as Try>::branch` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<Option<VaultEvidence> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<Option<[u64; 33]> as Try>::branch` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<Option<[u64; 4]> as Try>::branch` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<Option<[u64; 53]> as Try>::branch` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<Option<[u64; 9]> as Try>::branch` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<Option<[u8; 32]> as Try>::branch` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<[u64; 33] as IntoIterator>::into_iter` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<[u8; 32] as TryFrom<&[u8]>>::try_from` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<core::array::IntoIter<u64, 33> as Iterator>::next` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `<core::slice::Iter<'_, u8> as Iterator>::any` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `alloc::vec::Vec::as_slice` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `alloc::vec::Vec::extend_from_slice` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `alloc::vec::Vec::truncate` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `alloc::vec::Vec::with_capacity` in `microkernel::vaultevid::EvidenceRecord::read_bank`
callee `core::num::div_ceil` in `microkernel::vaultevid::EvidenceRecord::read_bank`
loop `microkernel::vaultevid::EvidenceRecord::read_bank` header bb39
callee `<&VaultEvidence as PartialEq>::eq` in `microkernel::vaultevid::eq`
callee `Formatter::debug_struct_field1_finish` in `microkernel::vaultevid::fmt`
callee `Formatter::debug_tuple_field1_finish` in `microkernel::vaultevid::fmt`
callee `Formatter::write_str` in `microkernel::vaultevid::fmt`
callee `<Option<Option<vcpu::Backend>> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::Kernel::ept_backend`
callee `<Result<bool, vcpu::HvError> as FromResidual<Result<Infallible, vcpu::HvError>>>::from_residual` in `microkernel::vcpu::Kernel::ept_scan_host_range`
callee `<Take<core::slice::Iter<'_, u64>> as Iterator>::copied` in `microkernel::vcpu::Kernel::hv_destroy_ept`
loop `microkernel::vcpu::Kernel::hv_destroy_ept` header bb15
callee `alloc::collections::BTreeMap::entry` in `microkernel::vcpu::Kernel::hv_new_ept`
callee `alloc::collections::btree_map::VacantEntry::insert` in `microkernel::vcpu::Kernel::hv_new_ept`
callee `<core::slice::Iter<'_, hypervisor::Range> as Iterator>::any` in `microkernel::vcpu::Kernel::hv_overlaps_protected`
callee `alloc::vec::Vec::push` in `microkernel::vcpu::Kernel::hv_protect`
callee `<Take<core::slice::Iter<'_, u64>> as Iterator>::copied` in `microkernel::vcpu::Kernel::hv_task_retired`
loop `microkernel::vcpu::Kernel::hv_task_retired` header bb7
callee `<Result<&Made, objects::ObjError> as Try>::branch` in `microkernel::vcpu::Kernel::retype_ept`
callee `<Result<synthyrion_capauth::SealedCap, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::vcpu::Kernel::retype_ept`
callee `<Result<alloc::vec::Vec<Made>, objects::ObjError> as Try>::branch` in `microkernel::vcpu::Kernel::retype_ept`
callee `<alloc::vec::Vec<Made> as Deref>::deref` in `microkernel::vcpu::Kernel::retype_ept`
callee `<Result<synthyrion_capauth::SealedCap, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::vcpu::Kernel::retype_ept_guest_on`
callee `<Result<synthyrion_capauth::SealedCap, objects::ObjError> as Try>::branch` in `microkernel::vcpu::Kernel::retype_ept_guest_on`
callee `<Result<synthyrion_capauth::SealedCap, objects::ObjError> as FromResidual<Result<Infallible, objects::ObjError>>>::from_residual` in `microkernel::vcpu::Kernel::retype_ept_on`
callee `<Result<synthyrion_capauth::SealedCap, objects::ObjError> as Try>::branch` in `microkernel::vcpu::Kernel::retype_ept_on`
callee `<Keys<'_, u64, vcpu::record::VcpuHome> as IntoIterator>::into_iter` in `microkernel::vcpu::Kernel::task_has_live_vcpu`
callee `<Keys<'_, u64, vcpu::record::VcpuHome> as Iterator>::next` in `microkernel::vcpu::Kernel::task_has_live_vcpu`
callee `alloc::collections::BTreeMap::keys` in `microkernel::vcpu::Kernel::task_has_live_vcpu`
loop `microkernel::vcpu::Kernel::task_has_live_vcpu` header bb3
callee `<vcpu::VcpuState as PartialEq>::ne` in `microkernel::vcpu::Kernel::task_has_live_vcpu::{closure#1}`
callee `<Option<vcpu::Backend> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::Kernel::vcpu_backend`
callee `<Result<(Option<u64>, Option<u64>), vcpu::HvError> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_bind`
callee `<Result<VcpuMeta, vcpu::HvError> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_bind`
callee `<vcpu::Backend as PartialEq>::ne` in `microkernel::vcpu::Kernel::vcpu_bind`
callee `<vcpu::VcpuState as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_bind`
callee `<Result<synthyrion_capauth::SealedCap, vcpu::HvError> as FromResidual<Result<Infallible, vcpu::HvError>>>::from_residual` in `microkernel::vcpu::Kernel::vcpu_create_on`
callee `alloc::collections::BTreeMap::remove` in `microkernel::vcpu::Kernel::vcpu_drop`
callee `<Inject as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `<Option<(Option<u64>, Option<u64>)> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `<Option<VcpuEntry> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `<Option<VcpuMeta> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `<Option<hypervisor::inject::IrqQueue> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `<Option<vcpu::VcpuRegs> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `<hypervisor::inject::EventKind as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `<vcpu::Backend as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `<vcpu::VcpuState as PartialEq>::ne` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `can_inject` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `hypervisor::inject::IrqQueue::accept` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `hypervisor::inject::IrqQueue::next` in `microkernel::vcpu::Kernel::vcpu_enter`
callee `<Option<vcpu::ExitInfo> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::Kernel::vcpu_exit_info`
callee `<vcpu::VcpuState as PartialEq>::ne` in `microkernel::vcpu::Kernel::vcpu_exited`
callee `<vcpu::Backend as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_exited_arm`
callee `hypervisor::stage2::to_neutral` in `microkernel::vcpu::Kernel::vcpu_exited_arm`
callee `<vcpu::Backend as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_exited_svm`
callee `hypervisor::svm::to_neutral` in `microkernel::vcpu::Kernel::vcpu_exited_svm`
callee `<Zip<core::slice::IterMut<'_, u64>, Keys<'_, u64, vcpu::record::VcpuHome>> as IntoIterator>::into_iter` in `microkernel::vcpu::Kernel::vcpu_ids`
callee `<Zip<core::slice::IterMut<'_, u64>, Keys<'_, u64, vcpu::record::VcpuHome>> as Iterator>::next` in `microkernel::vcpu::Kernel::vcpu_ids`
callee `alloc::collections::BTreeMap::keys` in `microkernel::vcpu::Kernel::vcpu_ids`
loop `microkernel::vcpu::Kernel::vcpu_ids` header bb5
callee `<Result<VcpuMeta, vcpu::HvError> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_inject`
callee `<vcpu::Backend as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_inject`
callee `<vcpu::VcpuState as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_inject`
callee `<Result<VcpuMeta, vcpu::HvError> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_inject_event`
callee `<hypervisor::inject::EventKind as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_inject_event`
callee `<vcpu::VcpuState as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_inject_event`
callee `amd_eventinj` in `microkernel::vcpu::Kernel::vcpu_inject_event`
callee `intel_entry_info` in `microkernel::vcpu::Kernel::vcpu_inject_event`
callee `<Result<VcpuMeta, vcpu::HvError> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_inject_virq`
callee `<vcpu::Backend as PartialEq>::ne` in `microkernel::vcpu::Kernel::vcpu_inject_virq`
callee `<vcpu::VcpuState as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_inject_virq`
callee `lr_pending` in `microkernel::vcpu::Kernel::vcpu_inject_virq`
callee `alloc::collections::BTreeMap::contains_key` in `microkernel::vcpu::Kernel::vcpu_of`
callee `<Option<Option<hypervisor::inject::Event>> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::Kernel::vcpu_pending_event`
callee `<vcpu::VcpuState as PartialEq>::eq` in `microkernel::vcpu::Kernel::vcpu_poll`
callee `<Option<vcpu::VcpuRegs> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::Kernel::vcpu_regs`
callee `<Result<VcpuMeta, vcpu::HvError> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_set_regs`
callee `<Result<(Option<u64>, Option<u64>), vcpu::HvError> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_start`
callee `<Result<VcpuMeta, vcpu::HvError> as Try>::branch` in `microkernel::vcpu::Kernel::vcpu_start`
callee `<vcpu::VcpuState as PartialEq>::ne` in `microkernel::vcpu::Kernel::vcpu_start`
callee `<Option<vcpu::VcpuState> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::Kernel::vcpu_state`
callee `<Take<core::slice::Iter<'_, u64>> as Iterator>::any` in `microkernel::vcpu::Kernel::vcpu_task_has_live`
callee `<vcpu::VcpuState as PartialEq>::ne` in `microkernel::vcpu::Kernel::vcpu_task_has_live::{closure#0}::{closure#1}`
callee `<vcpu::Backend as PartialEq>::ne` in `microkernel::vcpu::Kernel::vcpu_virq_take`
callee `<vcpu::VcpuState as PartialEq>::ne` in `microkernel::vcpu::Kernel::vcpu_virq_take`
callee `VirqBatch::empty` in `microkernel::vcpu::Kernel::vcpu_virq_take`
callee `VirqBatch::push` in `microkernel::vcpu::Kernel::vcpu_virq_take`
loop `microkernel::vcpu::Kernel::vcpu_virq_take` header bb12
callee `<RvH as Default>::default` in `microkernel::vcpu::default`
callee `<[u64; 15] as Default>::default` in `microkernel::vcpu::default`
callee `<u32 as Default>::default` in `microkernel::vcpu::default`
callee `<u64 as Default>::default` in `microkernel::vcpu::default`
callee `Cell::new` in `microkernel::vcpu::default`
callee `<&u64 as PartialEq>::eq` in `microkernel::vcpu::eq`
callee `<Inject as PartialEq>::eq` in `microkernel::vcpu::eq`
callee `<[u64; 15] as PartialEq>::eq` in `microkernel::vcpu::eq`
callee `<vcpu::Backend as PartialEq>::eq` in `microkernel::vcpu::eq`
callee `<vcpu::VcpuRegs as PartialEq>::eq` in `microkernel::vcpu::eq`
callee `Formatter::debug_struct_field1_finish` in `microkernel::vcpu::fmt`
callee `Formatter::debug_struct_field4_finish` in `microkernel::vcpu::fmt`
callee `Formatter::debug_struct_field5_finish` in `microkernel::vcpu::fmt`
callee `Formatter::write_str` in `microkernel::vcpu::fmt`
callee `<Option<VcpuMeta> as Try>::branch` in `microkernel::vcpu::record::Kernel::vcpu_event_of`
callee `<Option<hypervisor::inject::Event> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::record::Kernel::vcpu_event_of`
callee `<Option<hypervisor::inject::EventKind> as Try>::branch` in `microkernel::vcpu::record::Kernel::vcpu_event_of`
callee `<Option<VcpuMeta> as Try>::branch` in `microkernel::vcpu::record::Kernel::vcpu_exit_of`
callee `<Option<vcpu::ExitInfo> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::record::Kernel::vcpu_exit_of`
callee `alloc::collections::BTreeMap::get` in `microkernel::vcpu::record::Kernel::vcpu_frame`
loop `microkernel::vcpu::record::Kernel::vcpu_irq_count` header bb2
callee `<Option<hypervisor::inject::IrqQueue> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::record::Kernel::vcpu_irq_of`
callee `hypervisor::inject::IrqQueue::from_words` in `microkernel::vcpu::record::Kernel::vcpu_irq_of`
loop `microkernel::vcpu::record::Kernel::vcpu_irq_of` header bb4
callee `<hypervisor::inject::GuestIntrState as Default>::default` in `microkernel::vcpu::record::Kernel::vcpu_place`
callee `alloc::collections::BTreeMap::insert` in `microkernel::vcpu::record::Kernel::vcpu_place`
callee `hypervisor::inject::IrqQueue::words` in `microkernel::vcpu::record::Kernel::vcpu_put_irq`
loop `microkernel::vcpu::record::Kernel::vcpu_put_irq` header bb5
loop `microkernel::vcpu::record::Kernel::vcpu_put_regs` header bb9
callee `<Option<vcpu::VcpuRegs> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::record::Kernel::vcpu_regs_of`
loop `microkernel::vcpu::record::Kernel::vcpu_regs_of` header bb19
loop `microkernel::vcpu::record::Kernel::vcpu_virq_count` header bb2
loop `microkernel::vcpu::record::Kernel::vcpu_virq_insert` header bb2
callee `<(u8, u32) as PartialOrd>::lt` in `microkernel::vcpu::record::Kernel::vcpu_virq_take_best`
callee `<Option<(u32, u8)> as FromResidual<Option<Infallible>>>::from_residual` in `microkernel::vcpu::record::Kernel::vcpu_virq_take_best`
callee `<Option<(usize, u8, u32)> as Try>::branch` in `microkernel::vcpu::record::Kernel::vcpu_virq_take_best`
loop `microkernel::vcpu::record::Kernel::vcpu_virq_take_best` header bb2
callee `alloc::collections::BTreeMap::get` in `microkernel::vcpu::record::Kernel::vset`
callee `alloc::collections::BTreeMap::get` in `microkernel::vcpu::record::Kernel::vw`
callee `alloc::collections::BTreeMap::get` in `microkernel::vcpu::record::Kernel::vw_peek`
callee `<[u8; 40] as IndexMut<core::ops::Range<usize>>>::index_mut` in `microkernel::words`
callee `core::slice::copy_from_slice` in `microkernel::words`
loop `microkernel::words` header bb4
callee `core::iter::from_fn` in `microkernel::words_of`
callee `<Option<&[u8]> as Try>::branch` in `microkernel::words_of::{closure#0}`
callee `<Rev<core::slice::Iter<'_, u8>> as Iterator>::fold` in `microkernel::words_of::{closure#0}`
callee `<core::slice::Iter<'_, u8> as Iterator>::rev` in `microkernel::words_of::{closure#0}`
loop `microkernel::wr_record` header bb1
callee `<u64 as TryFrom<usize>>::try_from` in `microkernel::wr_word`
callee `<M as EvMem>::write` in `microkernel::write_block`
loop `microkernel::write_block` header bb4
CROSS-CHECK: 9 PENDING entries checked, 0 whose table root cause `missing` does not list

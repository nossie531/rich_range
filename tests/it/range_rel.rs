use rich_range::*;

#[test]
fn is_xxx() {
    let rud = RangeRel::Undefined;
    let req = RangeRel::Equal;
    let rbn = RangeRel::Before(true);
    let rbi = RangeRel::Before(false);
    let rmn = RangeRel::Meets(true);
    let rmi = RangeRel::Meets(false);
    let rsn = RangeRel::Starts(true);
    let rsi = RangeRel::Starts(false);
    let rfn = RangeRel::Finishes(true);
    let rfi = RangeRel::Finishes(false);
    let ron = RangeRel::Overlaps(true);
    let roi = RangeRel::Overlaps(false);
    let rdn = RangeRel::During(true);
    let rdi = RangeRel::During(false);
    // Vs methods for each variants (`true` pattern).
    assert!(rud.is_undefined());
    assert!(req.is_equal());
    assert!(rbn.is_bef_reg());
    assert!(rbi.is_bef_inv());
    assert!(rmn.is_met_reg());
    assert!(rmi.is_met_inv());
    assert!(rsn.is_stt_reg());
    assert!(rsi.is_stt_inv());
    assert!(rfn.is_fin_reg());
    assert!(rfi.is_fin_inv());
    assert!(ron.is_ovl_reg());
    assert!(roi.is_ovl_inv());
    assert!(rdn.is_dur_reg());
    assert!(rdi.is_dur_inv());
    // Vs methods for each variants (`false` pattern).
    assert!(!req.is_undefined());
    assert!(!rud.is_equal());
    assert!(!rud.is_bef_reg());
    assert!(!rud.is_bef_inv());
    assert!(!rud.is_met_reg());
    assert!(!rud.is_met_inv());
    assert!(!rud.is_stt_reg());
    assert!(!rud.is_stt_inv());
    assert!(!rud.is_fin_reg());
    assert!(!rud.is_fin_inv());
    assert!(!rud.is_ovl_reg());
    assert!(!rud.is_ovl_inv());
    assert!(!rud.is_dur_reg());
    assert!(!rud.is_dur_inv());
    // Vs methods that ignore normal or inverted (`true` pattern).
    assert!(rbn.is_bef_xxx());
    assert!(rbi.is_bef_xxx());
    assert!(rmn.is_met_xxx());
    assert!(rmi.is_met_xxx());
    assert!(rsn.is_stt_xxx());
    assert!(rsi.is_stt_xxx());
    assert!(rfn.is_fin_xxx());
    assert!(rfi.is_fin_xxx());
    assert!(ron.is_ovl_xxx());
    assert!(roi.is_ovl_xxx());
    assert!(rdn.is_dur_xxx());
    assert!(rdi.is_dur_xxx());
    // Vs methods that ignore normal or inverted (`false` pattern).
    assert!(!rud.is_bef_xxx());
    assert!(!rud.is_met_xxx());
    assert!(!rud.is_stt_xxx());
    assert!(!rud.is_fin_xxx());
    assert!(!rud.is_ovl_xxx());
    assert!(!rud.is_dur_xxx());
    // Vs methods `is_intersect`.
    assert!(!rud.is_intersects());
    assert!(req.is_intersects());
    assert!(!rbn.is_intersects());
    assert!(!rbi.is_intersects());
    assert!(!rmn.is_intersects());
    assert!(!rmi.is_intersects());
    assert!(rsn.is_intersects());
    assert!(rsi.is_intersects());
    assert!(rfn.is_intersects());
    assert!(rfi.is_intersects());
    assert!(ron.is_intersects());
    assert!(roi.is_intersects());
    assert!(rdn.is_intersects());
    assert!(rdi.is_intersects());
    // Vs methods `is_includes`.
    assert!(!rud.is_includes());
    assert!(req.is_includes());
    assert!(!rbn.is_includes());
    assert!(!rbi.is_includes());
    assert!(!rmn.is_includes());
    assert!(!rmi.is_includes());
    assert!(!rsn.is_includes());
    assert!(rsi.is_includes());
    assert!(!rfn.is_includes());
    assert!(rfi.is_includes());
    assert!(!ron.is_includes());
    assert!(!roi.is_includes());
    assert!(!rdn.is_includes());
    assert!(rdi.is_includes());
    // Vs methods `is_included`.
    assert!(!rud.is_included());
    assert!(req.is_included());
    assert!(!rbn.is_included());
    assert!(!rbi.is_included());
    assert!(!rmn.is_included());
    assert!(!rmi.is_included());
    assert!(rsn.is_included());
    assert!(!rsi.is_included());
    assert!(rfn.is_included());
    assert!(!rfi.is_included());
    assert!(!ron.is_included());
    assert!(!roi.is_included());
    assert!(rdn.is_included());
    assert!(!rdi.is_included());
}

#[test]
fn inverse() {
    let datas = [
        (RangeRel::Undefined, RangeRel::Undefined),
        (RangeRel::Equal, RangeRel::Equal),
        (RangeRel::Before(true), RangeRel::Before(false)),
        (RangeRel::Before(false), RangeRel::Before(true)),
        (RangeRel::Meets(true), RangeRel::Meets(false)),
        (RangeRel::Meets(false), RangeRel::Meets(true)),
        (RangeRel::Starts(true), RangeRel::Starts(false)),
        (RangeRel::Starts(false), RangeRel::Starts(true)),
        (RangeRel::Finishes(true), RangeRel::Finishes(false)),
        (RangeRel::Finishes(false), RangeRel::Finishes(true)),
        (RangeRel::Overlaps(true), RangeRel::Overlaps(false)),
        (RangeRel::Overlaps(false), RangeRel::Overlaps(true)),
        (RangeRel::During(true), RangeRel::During(false)),
        (RangeRel::During(false), RangeRel::During(true)),
    ];

    for (target, asis) in datas {
        let tobe = target.inverse();
        assert_eq!(asis, tobe);
    }
}

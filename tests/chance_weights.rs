/// Chance-node weighting: dealt cards come only from cards neither hand holds.
use dcfr_solver::card::*;
use dcfr_solver::cfr::*;
use dcfr_solver::game::*;
use dcfr_solver::range::Range;

#[test]
fn test_turn_checkdown_ev_is_exact_equity() {
    // No chips behind, so the hand checks down. AA wins unless the river is
    // one of the two remaining kings. With both hands known, the river comes
    // from 52 - 4 board - 4 hole = 44 cards, so EV(AA) = pot * 42/44.
    let mut board = Hand::new();
    for c in parse_cards("2c3d4h7s").unwrap() { board = board.add(c); }
    for use_iso in [true, false] {
        let config = SubgameConfig {
            board,
            pot: 100,
            stacks: [0, 0],
            ranges: [Range::parse("AA").unwrap(), Range::parse("KK").unwrap()],
            iterations: 1,
            street: Street::Turn,
            warmup_frac: 0.0,
            bet_config: None,
            dcfr: true,
            cfr_plus: true, skip_cum_strategy: false, dcfr_mode: DcfrMode::Standard,
            depth_limit: None, rake_pct: 0.0, rake_cap: 0.0, exploration_eps: 0.0, entropy_bonus: 0.0, entropy_anneal: false, entropy_root_only: false, opp_dilute: 0.0, softmax_temp: 0.0, current_iteration: 0, use_iso, rm_floor: 0.0, alternating: false, t_weight: false, frozen_root: None, check_bias: 0.0, pref_passive_delta: 1.0, pref_beta: 0.0, pref_beta_all_nodes: false, pruning: false, combo_check_bias: None, frozen_warmup: 0, unfreeze_decay: 1.0,
        };
        let mut solver = SubgameSolver::new(config);
        solver.solve();
        let aa = combo_index(parse_cards("Ah").unwrap()[0], parse_cards("As").unwrap()[0]) as usize;
        let kk = combo_index(parse_cards("Kh").unwrap()[0], parse_cards("Ks").unwrap()[0]) as usize;
        let ev_aa = solver.compute_ev(OOP)[aa];
        let ev_kk = solver.compute_ev(IP)[kk];
        assert!((ev_aa - 100.0 * 42.0 / 44.0).abs() < 1e-3, "use_iso={use_iso}: EV(AA) = {ev_aa}, want {}", 100.0 * 42.0 / 44.0);
        assert!((ev_kk - 100.0 * 2.0 / 44.0).abs() < 1e-3, "use_iso={use_iso}: EV(KK) = {ev_kk}, want {}", 100.0 * 2.0 / 44.0);
    }
}

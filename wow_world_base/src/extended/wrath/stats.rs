use crate::shared::player_race_tbc_wrath::PlayerRace;
use crate::wrath::exp::MAX_LEVEL;
use crate::wrath::{Class, RaceClass};

/// Calculate base melee attack power.
///
/// For druids this does not take into account the different attack power calculations for
/// the different forms. Use [`non_cat_form_base_attack_power`] or
/// [`cat_form_base_attack_power`] instead.
pub fn base_melee_attack_power(class: Class, strength: u16, agility: u16, level: u8) -> u32 {
    let level = level as u32;
    let strength = strength as u32;
    let agility = agility as u32;

    match class {
        Class::DeathKnight | Class::Paladin | Class::Warrior => {
            (level * 3 + strength * 2).saturating_sub(20)
        }
        Class::Shaman | Class::Hunter | Class::Rogue => {
            (level * 2 + strength + agility).saturating_sub(20)
        }
        Class::Druid => (strength * 2).saturating_sub(20),
        Class::Priest | Class::Mage | Class::Warlock => strength.saturating_sub(10),
    }
}

fn predatory_strikes(predatory_strikes_rank: u8, level: u8) -> f32 {
    let level = level as f32;

    if predatory_strikes_rank == 1 {
        level * 0.5
    } else if predatory_strikes_rank == 2 {
        level
    } else if predatory_strikes_rank == 3 {
        level * 1.5
    } else {
        0.0
    }
}

/// Calculate base attack power for druids in bear, dire bear, and moonkin forms.
///
/// Valid values for `predatory_strikes_rank` is 0, 1, 2, or 3.
/// All other values will be treated as 0.
///
/// Predatory Strikes is a feral druid talent that increases the attack power in
/// cat, bear, and dire bear forms by a percentage of their level.
///
/// Rank 1 (spell id 16972) increases by 50% of level.
///
/// Rank 2 (spell id 16974) increases by 100% of level.
///
/// Rank 3 (spell id 16975) increases by 150% of level.
///
/// This does not include the additional attack power from weapons that Predatory Strikes
/// also provides in Wrath.
pub fn non_cat_form_base_attack_power(strength: u16, level: u8, predatory_strikes_rank: u8) -> u32 {
    let strength = strength as f32;

    (predatory_strikes(predatory_strikes_rank, level) * strength * 2.0 - 20.0) as u32
}

/// Calculate base attack power for druids in cat form.
///
/// Valid values for `predatory_strikes_rank` is 0, 1, 2, or 3.
/// All other values will be treated as 0.
///
/// Predatory Strikes is a feral druid talent that increases the attack power in
/// cat, bear, and dire bear forms by a percentage of their level.
///
/// Rank 1 (spell id 16972) increases by 50% of level.
///
/// Rank 2 (spell id 16974) increases by 100% of level.
///
/// Rank 3 (spell id 16975) increases by 150% of level.
///
/// This does not include the additional attack power from weapons that Predatory Strikes
/// also provides in Wrath.
pub fn cat_form_base_attack_power(
    strength: u16,
    agility: u16,
    level: u8,
    predatory_strikes_rank: u8,
) -> u32 {
    let strength = strength as f32;
    let agility = agility as f32;

    (predatory_strikes(predatory_strikes_rank, level) * strength * 2.0 + agility - 20.0) as u32
}

/// Calculate base ranged attack power.
///
/// Since only warrior, rogue, and hunter can use weapons that use ranged attack power
/// all other classes return 0.
pub fn base_ranged_attack_power(class: Class, agility: u16, level: u8) -> u32 {
    let level = level as u32;
    let agility = agility as u32;

    match class {
        Class::Warrior | Class::Rogue => (level + agility).saturating_sub(10),
        Class::Hunter => (level * 2 + agility).saturating_sub(10),

        Class::DeathKnight
        | Class::Paladin
        | Class::Priest
        | Class::Shaman
        | Class::Mage
        | Class::Warlock
        | Class::Druid => 0,
    }
}

// Also found in the `gtChanceToMeleeCrit` .dbc.
// In there it's an array that can be indexed by `(class.as_int() - 1) * 100 + (level - 1)`.
// Careful that level 0 does not wrap around.
#[allow(clippy::excessive_precision)]
const fn crit_ratio(class: Class, level: u8) -> f32 {
    let level_index = if level == 0 {
        0
    } else if level > MAX_LEVEL {
        MAX_LEVEL - 1
    } else {
        level - 1
    } as usize;

    const WARRIOR_AND_DEATH_KNIGHT_RATIO: &[f32; 80] = &[
        0.00258699990808964,
        0.00226400000974536,
        0.00226400000974536,
        0.00226400000974536,
        0.00226400000974536,
        0.00201199995353818,
        0.00201199995353818,
        0.00201199995353818,
        0.00201199995353818,
        0.00201199995353818,
        0.00181100005283952,
        0.00181100005283952,
        0.00164599996060133,
        0.00164599996060133,
        0.00150899996515363,
        0.00150899996515363,
        0.00150899996515363,
        0.00139300001319498,
        0.00139300001319498,
        0.00129299995023757,
        0.00129299995023757,
        0.00129299995023757,
        0.00120699999388307,
        0.00113200000487268,
        0.00113200000487268,
        0.00106499996036291,
        0.00106499996036291,
        0.00100599997676909,
        0.00100599997676909,
        0.000952999980654567,
        0.000952999980654567,
        0.000905000022612512,
        0.000905000022612512,
        0.000861999986227602,
        0.000861999986227602,
        0.000822999980300665,
        0.000822999980300665,
        0.000786999997217208,
        0.000786999997217208,
        0.000754999986384064,
        0.000723999983165413,
        0.000723999983165413,
        0.000696000002790242,
        0.000696000002790242,
        0.000670999987050891,
        0.000670999987050891,
        0.000646999978926033,
        0.000623999978415668,
        0.000623999978415668,
        0.000604000000748783,
        0.000604000000748783,
        0.000584000023081899,
        0.00056600000243634,
        0.00056600000243634,
        0.000548999989405274,
        0.000548999989405274,
        0.000532999983988702,
        0.00051699997857213,
        0.00051699997857213,
        0.000502999988384545,
        0.00047699999413453,
        0.000452999986009672,
        0.000430999993113801,
        0.000421000004280359,
        0.000402000005124137,
        0.000384999992093071,
        0.000369999994290993,
        0.000354999996488914,
        0.000342000013915822,
        0.000334999989718199,
        0.000311999989207834,
        0.000287000002572313,
        0.000265999988187104,
        0.000247999996645376,
        0.000232000005780719,
        0.000216000000364147,
        0.000199000001884997,
        0.000184999997145496,
        0.000172000000020489,
        0.00015999999595806,
    ];

    let ratio = match class {
        Class::Warrior => WARRIOR_AND_DEATH_KNIGHT_RATIO,

        Class::Paladin => &[
            0.00216399994678795,
            0.00216399994678795,
            0.00216399994678795,
            0.00192399998195469,
            0.00192399998195469,
            0.00192399998195469,
            0.00192399998195469,
            0.00173200003337115,
            0.00173200003337115,
            0.00173200003337115,
            0.00173200003337115,
            0.00173200003337115,
            0.00157399999443442,
            0.00157399999443442,
            0.00144300004467368,
            0.00144300004467368,
            0.00144300004467368,
            0.00133200001437217,
            0.00133200001437217,
            0.00123699998948723,
            0.00123699998948723,
            0.00123699998948723,
            0.00115400005597621,
            0.00108199997339398,
            0.00108199997339398,
            0.00108199997339398,
            0.00101899995934218,
            0.00101899995934218,
            0.000961999990977347,
            0.000961999990977347,
            0.000911000010091811,
            0.000911000010091811,
            0.000866000016685575,
            0.000866000016685575,
            0.000824999995529652,
            0.000824999995529652,
            0.000824999995529652,
            0.000786999997217208,
            0.000786999997217208,
            0.000752999971155077,
            0.000752999971155077,
            0.000752999971155077,
            0.000721000018529594,
            0.000692999979946762,
            0.000692999979946762,
            0.000666000007186085,
            0.000666000007186085,
            0.000640999991446733,
            0.000640999991446733,
            0.000617999990936369,
            0.000597000005654991,
            0.000597000005654991,
            0.000577000027988106,
            0.000577000027988106,
            0.000559000007342547,
            0.000559000007342547,
            0.000540999986696988,
            0.000524999981280416,
            0.000524999981280416,
            0.000508999975863844,
            0.000494999985676259,
            0.000480999995488673,
            0.000468000012915581,
            0.000456000008853152,
            0.000444000004790723,
            0.000444000004790723,
            0.000422000011894852,
            0.000422000011894852,
            0.000411999993957579,
            0.00040300001273863,
            0.000368000008165836,
            0.000345999986166134,
            0.000320999999530613,
            0.000299000006634742,
            0.000274999998509884,
            0.000257999985478818,
            0.00023999999393709,
            0.000222000002395362,
            0.00020599999697879,
            0.000192000006791204,
        ],

        Class::Hunter => &[
            0.00283999997191131,
            0.00283399992622435,
            0.00271099992096424,
            0.0025299999397248,
            0.00243000010959804,
            0.00233700009994209,
            0.00225099991075695,
            0.00217100000008941,
            0.00205100001767278,
            0.00198399997316301,
            0.00184799998532981,
            0.00167000002693385,
            0.00154700002167374,
            0.00144100002944469,
            0.00132999999914318,
            0.00126699998509139,
            0.00119400001130998,
            0.0011170000070706,
            0.00106000003870577,
            0.000998000032268465,
            0.000961999990977347,
            0.000910000002477318,
            0.000872000004164875,
            0.000829000025987625,
            0.000797000015154481,
            0.000767000019550323,
            0.000734000001102686,
            0.000708999985363334,
            0.00067999999737367,
            0.000653999974019825,
            0.000637000019196421,
            0.000614000018686056,
            0.000592000025790185,
            0.000575000012759119,
            0.000555999984499067,
            0.000540999986696988,
            0.000523999973665923,
            0.000508000026457012,
            0.000493000028654933,
            0.000480999995488673,
            0.000469999999040738,
            0.000456999987363815,
            0.000444000004790723,
            0.000433000008342788,
            0.000421000004280359,
            0.000413000001572073,
            0.000402000005124137,
            0.000391000008676201,
            0.000381999998353422,
            0.000372999988030642,
            0.00036599999293685,
            0.000357999990228564,
            0.000349999987520278,
            0.000341000006301329,
            0.000334000011207536,
            0.000327999994624406,
            0.000320999999530613,
            0.000314000004436821,
            0.000307000009343028,
            0.000300999992759898,
            0.000296999991405755,
            0.000289999996311963,
            0.000284000008832663,
            0.000278999999864027,
            0.000273000012384728,
            0.000269999989541247,
            0.000264000002061948,
            0.000258999993093312,
            0.000254000013228506,
            0.000250000011874363,
            0.000232000005780719,
            0.000216000000364147,
            0.000201000002562068,
            0.000186999997822568,
            0.000172999993083067,
            0.000161000003572553,
            0.000150000007124618,
            0.000138999996124767,
            0.000128999992739409,
            0.000119999996968545,
        ],

        Class::Rogue => &[
            0.00447600008919835,
            0.00429000006988645,
            0.00411800015717745,
            0.00381299993023276,
            0.00367700005881488,
            0.0035500000230968,
            0.00332099990919232,
            0.00321700004860759,
            0.00312000000849366,
            0.00294100004248321,
            0.00264000007882714,
            0.00239400006830692,
            0.00214500003494322,
            0.00197999994270504,
            0.0017750000115484,
            0.00165999995078892,
            0.00156000000424683,
            0.00144999998155981,
            0.00135499995667487,
            0.00127100001554936,
            0.00119700003415346,
            0.00114399997983128,
            0.00108399998862296,
            0.00104000000283122,
            0.000980000011622906,
            0.000936000025831163,
            0.000903000007383525,
            0.000865000009071082,
            0.000829999975394458,
            0.000791999977082014,
            0.000768000027164817,
            0.000740999996196479,
            0.000714999972842634,
            0.000691000022925436,
            0.000663999991957098,
            0.00064300000667572,
            0.000628000008873641,
            0.000608999980613589,
            0.000592000025790185,
            0.000571999989915639,
            0.000555999984499067,
            0.000541999994311482,
            0.000528000004123896,
            0.000511999998707324,
            0.000497000000905246,
            0.00048600000445731,
            0.000474000000394881,
            0.000464000011561438,
            0.000453999993624166,
            0.00044000000343658,
            0.000430999993113801,
            0.000422000011894852,
            0.000411999993957579,
            0.000403999991249293,
            0.000394000002415851,
            0.000385999999707565,
            0.000377999996999279,
            0.000369999994290993,
            0.000364000006811693,
            0.000354999996488914,
            0.000334000011207536,
            0.000322000007145107,
            0.000307000009343028,
            0.000296000012895092,
            0.00028599999495782,
            0.000276000006124377,
            0.000268000003416091,
            0.000261999986832961,
            0.000255999999353662,
            0.000250000011874363,
            0.000232000005780719,
            0.000216000000364147,
            0.000201000002562068,
            0.000186999997822568,
            0.000172999993083067,
            0.000161000003572553,
            0.000150000007124618,
            0.000138999996124767,
            0.000128999992739409,
            0.000119999996968545,
        ],

        Class::Priest => &[
            0.000912000017706305,
            0.000912000017706305,
            0.000912000017706305,
            0.000867999973706901,
            0.000867999973706901,
            0.000867999973706901,
            0.000867999973706901,
            0.000829000025987625,
            0.000829000025987625,
            0.000829000025987625,
            0.000829000025987625,
            0.000792999984696507,
            0.000792999984696507,
            0.000792999984696507,
            0.000792999984696507,
            0.000760000024456531,
            0.000760000024456531,
            0.000760000024456531,
            0.00072900002123788,
            0.00072900002123788,
            0.00072900002123788,
            0.00072900002123788,
            0.000700999982655048,
            0.000700999982655048,
            0.000700999982655048,
            0.000675000017508864,
            0.000675000017508864,
            0.000675000017508864,
            0.000651000009384006,
            0.000651000009384006,
            0.000651000009384006,
            0.000629000016488135,
            0.000629000016488135,
            0.000629000016488135,
            0.000607999972999096,
            0.000607999972999096,
            0.000607999972999096,
            0.000587999995332211,
            0.000587999995332211,
            0.000587999995332211,
            0.000569999974686652,
            0.000569999974686652,
            0.000553000019863248,
            0.000553000019863248,
            0.000553000019863248,
            0.000536000006832182,
            0.000536000006832182,
            0.000521000009030104,
            0.000521000009030104,
            0.000521000009030104,
            0.000507000018842518,
            0.000507000018842518,
            0.000493000028654933,
            0.000493000028654933,
            0.00047999998787418,
            0.00047999998787418,
            0.000468000012915581,
            0.000468000012915581,
            0.000456000008853152,
            0.000456000008853152,
            0.000445000012405217,
            0.00044599999091588,
            0.00044299999717623,
            0.000433999986853451,
            0.000426999991759658,
            0.000421000004280359,
            0.000414999987697229,
            0.000413000001572073,
            0.000411999993957579,
            0.000400999997509643,
            0.000372000009519979,
            0.000344000000040978,
            0.00031999999191612,
            0.000299000006634742,
            0.000276000006124377,
            0.000257000006968156,
            0.00023999999393709,
            0.000222000002395362,
            0.000207000004593283,
            0.000192000006791204,
        ],

        Class::DeathKnight => WARRIOR_AND_DEATH_KNIGHT_RATIO,

        Class::Shaman => &[
            0.00103899999521673,
            0.00103899999521673,
            0.000989999971352518,
            0.000989999971352518,
            0.000944999977946281,
            0.000944999977946281,
            0.000944999977946281,
            0.000903000007383525,
            0.000903000007383525,
            0.000866000016685575,
            0.000866000016685575,
            0.000830999983008951,
            0.000830999983008951,
            0.000798999972175807,
            0.000769999984186143,
            0.000742000003810972,
            0.000742000003810972,
            0.00071699998807162,
            0.00071699998807162,
            0.000669999979436398,
            0.000669999979436398,
            0.00064899999415502,
            0.00064899999415502,
            0.000630000024102628,
            0.000610999995842576,
            0.000593999982811511,
            0.000593999982811511,
            0.000577000027988106,
            0.000577000027988106,
            0.000546999974176288,
            0.000546999974176288,
            0.000532999983988702,
            0.00052000000141561,
            0.00052000000141561,
            0.000494999985676259,
            0.00048300001071766,
            0.00048300001071766,
            0.000472000014269724,
            0.000472000014269724,
            0.000452000007499009,
            0.000441999989561737,
            0.000441999989561737,
            0.000433000008342788,
            0.000423999998020008,
            0.000415999995311722,
            0.000407000014092773,
            0.00039999998989515,
            0.000391999987186864,
            0.000391999987186864,
            0.000377999996999279,
            0.000371000001905486,
            0.000365000014426187,
            0.000365000014426187,
            0.000357999990228564,
            0.000345999986166134,
            0.000341000006301329,
            0.000334999989718199,
            0.000334999989718199,
            0.000330000009853393,
            0.00031999999191612,
            0.000310000003082678,
            0.000303999986499548,
            0.000293999997666106,
            0.000284999987343326,
            0.000280999985989183,
            0.000273000012384728,
            0.000266999995801598,
            0.000261000008322299,
            0.000254999991739169,
            0.000250000011874363,
            0.000232000005780719,
            0.000216000000364147,
            0.000201000002562068,
            0.000186999997822568,
            0.000172999993083067,
            0.000161000003572553,
            0.000150000007124618,
            0.000138999996124767,
            0.000128999992739409,
            0.000119999996968545,
        ],

        Class::Mage => &[
            0.000773000007029623,
            0.000773000007029623,
            0.000773000007029623,
            0.000736000016331673,
            0.000736000016331673,
            0.000736000016331673,
            0.000736000016331673,
            0.000736000016331673,
            0.000736000016331673,
            0.000702999997884035,
            0.000702999997884035,
            0.000702999997884035,
            0.000702999997884035,
            0.000702999997884035,
            0.000671999994665384,
            0.000671999994665384,
            0.000671999994665384,
            0.000671999994665384,
            0.000671999994665384,
            0.000644000014290214,
            0.000644000014290214,
            0.000644000014290214,
            0.000644000014290214,
            0.000617999990936369,
            0.000617999990936369,
            0.000617999990936369,
            0.000617999990936369,
            0.000617999990936369,
            0.000594999990426004,
            0.000594999990426004,
            0.000594999990426004,
            0.000594999990426004,
            0.000572999997530133,
            0.000572999997530133,
            0.000572999997530133,
            0.000552000012248755,
            0.000552000012248755,
            0.000552000012248755,
            0.000552000012248755,
            0.000532999983988702,
            0.000532999983988702,
            0.000532999983988702,
            0.000532999983988702,
            0.000515000021550804,
            0.000515000021550804,
            0.000515000021550804,
            0.000499000016134232,
            0.000499000016134232,
            0.000499000016134232,
            0.00048300001071766,
            0.00048300001071766,
            0.00048300001071766,
            0.000468000012915581,
            0.000468000012915581,
            0.000468000012915581,
            0.000455000001238659,
            0.000455000001238659,
            0.000455000001238659,
            0.000441999989561737,
            0.000441999989561737,
            0.000441999989561737,
            0.000441999989561737,
            0.000429000006988645,
            0.000429000006988645,
            0.000429000006988645,
            0.000418000010540709,
            0.000418000010540709,
            0.000418000010540709,
            0.000407000014092773,
            0.000407000014092773,
            0.000376999989384785,
            0.000350999995134771,
            0.000329000002238899,
            0.000303000007988885,
            0.000280999985989183,
            0.000261999986832961,
            0.000241999994614162,
            0.000226999996812083,
            0.000209000005270354,
            0.000195999993593432,
        ],

        Class::Warlock => &[
            0.00118899997323751,
            0.00118899997323751,
            0.00113200000487268,
            0.00113200000487268,
            0.00113200000487268,
            0.00108099996577948,
            0.00108099996577948,
            0.00108099996577948,
            0.00103399995714426,
            0.00103399995714426,
            0.000990999978967011,
            0.000990999978967011,
            0.000990999978967011,
            0.000959000026341528,
            0.000944000028539449,
            0.000928000023122877,
            0.000913999974727631,
            0.000898999976925552,
            0.000884999986737967,
            0.000870999996550381,
            0.000857000006362796,
            0.000844000023789704,
            0.000830999983008951,
            0.000818000000435859,
            0.000805000017862767,
            0.000791999977082014,
            0.000780000002123415,
            0.000768000027164817,
            0.000755999993998557,
            0.000745000026654452,
            0.000732999993488193,
            0.000722000026144087,
            0.000711000000592321,
            0.000699999975040555,
            0.000690000015310943,
            0.000678999989759177,
            0.000668999971821904,
            0.000659000012092292,
            0.00064899999415502,
            0.000638999976217747,
            0.000630000024102628,
            0.000620000006165355,
            0.000610999995842576,
            0.000601999985519797,
            0.000592999975197017,
            0.000584000023081899,
            0.000576000020373613,
            0.000567000010050833,
            0.000559000007342547,
            0.000551000004634261,
            0.000543000001925975,
            0.000534999999217689,
            0.000526999996509403,
            0.000518999993801117,
            0.000511999998707324,
            0.000503999995999038,
            0.000497000000905246,
            0.000490000005811453,
            0.00048300001071766,
            0.000475999986520037,
            0.000468999991426244,
            0.000461999996332452,
            0.000455000001238659,
            0.00044900001375936,
            0.000441999989561737,
            0.000436000002082437,
            0.000429999985499308,
            0.000423999998020008,
            0.000418000010540709,
            0.000411999993957579,
            0.000384000013582408,
            0.000354999996488914,
            0.000330000009853393,
            0.000308999995468184,
            0.000287000002572313,
            0.000264000002061948,
            0.000245000002905726,
            0.000228999997489154,
            0.000211999999010004,
            0.000197999994270504,
        ],

        Class::Druid => &[
            0.00126199994701892,
            0.00126199994701892,
            0.00120199995581061,
            0.00120199995581061,
            0.00114800001028925,
            0.00114800001028925,
            0.00109799997881055,
            0.00109799997881055,
            0.00105199997778982,
            0.000971000001300126,
            0.00093500001821667,
            0.00093500001821667,
            0.000901999999769032,
            0.000901999999769032,
            0.000842000008560717,
            0.000842000008560717,
            0.000814000028185546,
            0.000789000012446195,
            0.000789000012446195,
            0.000700999982655048,
            0.000700999982655048,
            0.000682000012602657,
            0.000663999991957098,
            0.000663999991957098,
            0.000630999973509461,
            0.000630999973509461,
            0.000615999975707382,
            0.000600999977905303,
            0.000600999977905303,
            0.000548999989405274,
            0.000537000014446676,
            0.000537000014446676,
            0.00052599998889491,
            0.000515000021550804,
            0.000505000003613532,
            0.000494999985676259,
            0.000484999996842816,
            0.000484999996842816,
            0.000475999986520037,
            0.00044299999717623,
            0.000434999994467944,
            0.000434999994467944,
            0.000427999999374151,
            0.000421000004280359,
            0.000407000014092773,
            0.000400999997509643,
            0.000400999997509643,
            0.000394000002415851,
            0.000387999985832721,
            0.00036599999293685,
            0.000361000013072044,
            0.000356000004103407,
            0.000350999995134771,
            0.000350999995134771,
            0.000341000006301329,
            0.000337000004947186,
            0.000331999995978549,
            0.000327999994624406,
            0.000323999993270263,
            0.000307999987853691,
            0.000299000006634742,
            0.000295000005280599,
            0.000284999987343326,
            0.000278999999864027,
            0.000273999990895391,
            0.000269000011030585,
            0.000265000009676442,
            0.000257999985478818,
            0.000254000013228506,
            0.000250000011874363,
            0.000232000005780719,
            0.000216000000364147,
            0.000201000002562068,
            0.000186999997822568,
            0.000172999993083067,
            0.000161000003572553,
            0.000150000007124618,
            0.000138999996124767,
            0.000128999992739409,
            0.000119999996968545,
        ],
    };

    ratio[level_index]
}

/// Calculate base melee crit from agility.
///
/// Does not return the chance as a percentage, but as a whole number.
/// So a 4% chance to crit would return 4.0.
#[allow(clippy::excessive_precision)]
pub fn base_melee_crit(class: Class, agility: u16, level: u8) -> f32 {
    let base: f32 = match class {
        Class::Warrior => 0.031890999525785446,
        Class::Paladin => 0.03268500044941902,
        Class::Hunter => -0.015320000238716602,
        Class::Rogue => -0.002950000111013651,
        Class::Priest => 0.03176499903202057,
        Class::DeathKnight => 0.031890999525785446,
        Class::Shaman => 0.02921999990940094,
        Class::Mage => 0.034540001302957535,
        Class::Warlock => 0.02621999941766262,
        Class::Druid => 0.074754998087883,
    };

    let ratio = crit_ratio(class, level);

    let crit = base + agility as f32 * ratio;
    crit * 100.0
}

/// Calculate base dodge chance from agility.
///
/// *Does* include the 2% from the night elf Quickness racial (skill id 20582)
/// and the base class dodge chances.
pub fn base_dodge_chance(class: Class, race: PlayerRace, agility: u16, level: u8) -> f32 {
    let class_base: f32 = match class {
        Class::Warrior => 0.0075,
        Class::Paladin => 0.00652,
        Class::Hunter => -0.0545,
        Class::Rogue => -0.0059,
        Class::Priest => 0.03183,
        Class::DeathKnight => 0.0114,
        Class::Shaman => 0.0167,
        Class::Mage => 0.034575,
        Class::Warlock => 0.02011,
        Class::Druid => -0.0187,
    };

    let crit_per_agility_to_dodge_per_agility_coefficient: f32 = match class {
        Class::Warrior => 1.1,
        Class::Paladin => 1.0,
        Class::Hunter => 1.6,
        Class::Rogue => 2.0,
        Class::Priest => 1.0,
        Class::DeathKnight => 1.0,
        Class::Shaman => 1.0,
        Class::Mage => 1.0,
        Class::Warlock => 1.0,
        Class::Druid => 1.7,
    };

    let dodge_ratio = crit_ratio(class, level) * crit_per_agility_to_dodge_per_agility_coefficient;

    let dodge = class_base + agility as f32 * dodge_ratio;

    /// 2% from Quickness racial (skill id 20582)
    let racial_bonus: f32 = if matches!(race, PlayerRace::NightElf) {
        2.0
    } else {
        0.0
    };

    (dodge * 100.0) + racial_bonus
}

impl RaceClass {
    /// Calculate base melee attack power.
    ///
    /// For druids this does not take into account the different attack power calculations for
    /// the different forms. Use [`non_cat_form_base_attack_power`] or
    /// [`cat_form_base_attack_power`] instead.
    pub fn base_melee_attack_power(&self, strength: u16, agility: u16, level: u8) -> u32 {
        base_melee_attack_power(self.class(), strength, agility, level)
    }

    /// Calculate base ranged attack power.
    ///
    /// Since only warrior, rogue, and hunter can use weapons that use ranged attack power
    /// all other classes return 0.
    pub fn base_ranged_attack_power(&self, agility: u16, level: u8) -> u32 {
        base_ranged_attack_power(self.class(), agility, level)
    }

    /// Calculate base melee crit from agility.
    ///
    /// Does not return the chance as a percentage, but as a whole number.
    /// So a 4% chance to crit would return 4.0.
    pub fn base_melee_crit(&self, agility: u16, level: u8) -> f32 {
        base_melee_crit(self.class(), agility, level)
    }

    /// Calculate base dodge chance from agility.
    ///
    /// *Does* include the 2% from the night elf Quickness racial (skill id 20582)
    /// and the base class dodge chances.
    pub fn base_dodge_chance(&self, agility: u16, level: u8) -> f32 {
        base_dodge_chance(self.class(), self.race(), agility, level)
    }
}

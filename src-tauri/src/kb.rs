//! 知识库匹配引擎：客服话术分级匹配
use serde::{Serialize, Deserialize};

/// 一条话术模板
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KbTemplate {
    pub id: u32,
    pub title: String,
    pub text: String,
    pub keywords: Vec<String>,
    pub category: String,
    pub match_type: String,
}

/// 获取全部知识库条目
pub fn all_templates() -> Vec<KbTemplate> {
    vec![
        // 售后退换
        KbTemplate{id:1, title:"7天无理由退货指引", keywords:vec!["退货","怎么退","退货流程"],
            text:"亲，我们支持 7 天无理由退货。请您保证商品及包装配件齐全，寄回至杭州仓：杭州市余杭区仓前路 128 号。收到后 1-2 个工作日处理退款。",
            category:"售后退换", match_type:""},
        KbTemplate{id:2, title:"退款到账时效说明", keywords:vec!["到账","什么时候到","退款多久"],
            text:"亲，退款一般在商家确认收货后的 1-3 个工作日原路退回您的支付账户。",
            category:"售后退换", match_type:""},
        KbTemplate{id:3, title:"换货流程建议（最佳方案）", keywords:vec!["换货","换颜色","换个"],
            text:"亲，建议您直接申请退货退款，然后重新拍下心仪款式。发货速度更快。",
            category:"售后退换", match_type:""},
        KbTemplate{id:31, title:"换货操作详细步骤", keywords:vec!["换货怎么操作","怎么换"],
            text:"亲，换货请按：① 订单详情点击「换货」→ ② 选择原因 → ③ 提交后收到二维码 → ④ 快递员上门取货。",
            category:"售后退换", match_type:""},
        // 物流咨询
        KbTemplate{id:4, title:"快递停滞加急催派", keywords:vec!["催件","停滞","没动静","还没到","卡住"],
            text:"非常抱歉给您带来困扰！我已帮您联系物流专员进行加急催派处理，预计今天内会有更新。",
            category:"物流咨询", match_type:""},
        KbTemplate{id:5, title:"发货快递与时效说明", keywords:vec!["发什么快递","哪家快递","快递公司","发货时间"],
            text:"亲，本店默认中通/圆通快递，如需顺丰请联系客服补差价。商品一般 24 小时内发货。",
            category:"物流咨询", match_type:""},
        KbTemplate{id:6, title:"物流追踪说明", keywords:vec!["到哪里了","到哪了","物流进度"],
            text:"亲，您可复制订单号到快递官网查看实时进度。我这边也帮您同步关注物流更新。",
            category:"物流咨询", match_type:""},
        // 售后投诉
        KbTemplate{id:7, title:"投诉升级路径", keywords:vec!["投诉","不满意","售后怎么办"],
            text:"非常理解您的心情。我已将您的情况升级至客服主管处理，您将在 2 小时内收到主管专属回复。",
            category:"售后投诉", match_type:""},
        KbTemplate{id:8, title:"售后安抚致歉", keywords:vec!["售后","不好意思","抱歉","对不起"],
            text:"非常抱歉给您带来不好的体验！我们一定会妥善解决您的问题，请告知具体情况，我第一时间帮您协调处理。",
            category:"售后投诉", match_type:""}
    ]
}

/// 三级匹配：热词直出 → 分类展开 → 无匹配
pub fn match(text: &str) -> Vec<KbTemplate> {
    let all = all_templates();
    let lower = text.to_lowercase();
    let mut result = Vec::<KbTemplate>::new();
    let mut seen = std::collections::HashSet::<u32>::new();

    // 1. 热词精确匹配
    for tpl in &all {
        for kw in &tpl.keywords {
            if lower.contains(kw.to_lowercase()) && !seen.contains(&tpl.id) {
                seen.insert(tpl.id);
                result.push(KbTemplate{
                    id: tpl.id, title: tpl.title.clone(), text: tpl.text.clone(),
                    keywords: tpl.keywords.clone(), category: tpl.category.clone(),
                    match_type: "hotword"
                });
                break;
            }
        }
    }

    if !result.is_empty() {
        return result[..5.min(result.len())].to_vec();
    }

    // 2. 分类别名匹配
    let categories: Vec<(String,Vec<String>)> = vec![
        ("售后退换", vec!["售后","退换","退款","退货"]),
        ("物流咨询", vec!["物流","快递","派送","追踪","发货"]),
        ("售后投诉", vec!["售后","投诉","不满","不满意"])
    ];
    for (cat_name, aliases) in &categories {
        let matched = lower.contains(cat_name.to_lowercase())
            || aliases.iter().any(|a| lower.contains(a.to_lowercase()));
        if !matched { continue; }
        for tpl in &all {
            if tpl.category == cat_name && !seen.contains(&tpl.id) {
                seen.insert(tpl.id);
                result.push(KbTemplate{
                    id: tpl.id, title: tpl.title.clone(), text: tpl.text.clone(),
                    keywords: tpl.keywords.clone(), category: tpl.category.clone(),
                    match_type: "category"
                });
            }
        }
    }

    result[..5.min(result.len())].to_vec()
}
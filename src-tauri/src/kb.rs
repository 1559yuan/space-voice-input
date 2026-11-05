use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KbTemplate {
    pub id: u32,
    pub title: String,
    pub text: String,
    pub keywords: Vec<String>,
    pub category: String,
    pub match_type: String,
}

fn s(val: &str) -> String { String::from(val) }

pub fn all_templates() -> Vec<KbTemplate> {
    vec![
        KbTemplate{id:1,
            title: s("7天无理由退货指引"),
            keywords: vec![s("退货"), s("怎么退"), s("退货流程")],
            text: s("亲，我们支持 7 天无理由退货。请您保证商品及包装配件齐全，寄回至杭州仓：杭州市余杭区仓前路 128 号。收到后 1-2 个工作日处理退款。"),
            category: s("售后退换"), match_type: s("")},
        KbTemplate{id:2,
            title: s("退款到账时效说明"),
            keywords: vec![s("到账"), s("什么时候到"), s("退款多久")],
            text: s("亲，退款一般在商家确认收货后的 1-3 个工作日原路退回您的支付账户。"),
            category: s("售后退换"), match_type: s("")},
        KbTemplate{id:3,
            title: s("换货流程建议（最佳方案）"),
            keywords: vec![s("换货"), s("换颜色"), s("换个")],
            text: s("亲，建议您直接申请退货退款，然后重新拍下心仪款式。发货速度更快。"),
            category: s("售后退换"), match_type: s("")},
        KbTemplate{id:31,
            title: s("换货操作详细步骤"),
            keywords: vec![s("换货怎么操作"), s("怎么换")],
            text: s("亲，换货请按：① 订单详情点击「换货」→ ② 选择原因 → ③ 提交后收到二维码 → ④ 快递员上门取货。"),
            category: s("售后退换"), match_type: s("")},
        KbTemplate{id:4,
            title: s("快递停滞加急催派"),
            keywords: vec![s("催件"), s("停滞"), s("没动静"), s("还没到"), s("卡住")],
            text: s("非常抱歉给您带来困扰！我已帮您联系物流专员进行加急催派处理，预计今天内会有更新。"),
            category: s("物流咨询"), match_type: s("")},
        KbTemplate{id:5,
            title: s("发货快递与时效说明"),
            keywords: vec![s("发什么快递"), s("哪家快递"), s("快递公司"), s("发货时间")],
            text: s("亲，本店默认中通/圆通快递，如需顺丰请联系客服补差价。商品一般 24 小时内发货。"),
            category: s("物流咨询"), match_type: s("")},
        KbTemplate{id:6,
            title: s("物流追踪说明"),
            keywords: vec![s("到哪里了"), s("到哪了"), s("物流进度")],
            text: s("亲，您可复制订单号到快递官网查看实时进度。我这边也帮您同步关注物流更新。"),
            category: s("物流咨询"), match_type: s("")},
        KbTemplate{id:7,
            title: s("投诉升级路径"),
            keywords: vec![s("投诉"), s("不满意"), s("售后怎么办")],
            text: s("非常理解您的心情。我已将您的情况升级至客服主管处理，您将在 2 小时内收到主管专属回复。"),
            category: s("售后投诉"), match_type: s("")},
        KbTemplate{id:8,
            title: s("售后安抚致歉"),
            keywords: vec![s("售后"), s("不好意思"), s("抱歉"), s("对不起")],
            text: s("非常抱歉给您带来不好的体验！我们一定会妥善解决您的问题，请告知具体情况，我第一时间帮您协调处理。"),
            category: s("售后投诉"), match_type: s("")},
    ]
}

pub fn match(text: &str) -> Vec<KbTemplate> {
    let all = all_templates();
    let lower = text.to_lowercase();
    let mut result = Vec::<KbTemplate>::new();
    let mut seen = std::collections::HashSet::<u32>::new();

    for tpl in &all {
        for kw in &tpl.keywords {
            if lower.contains(kw.to_lowercase()) && !seen.contains(&tpl.id) {
                seen.insert(tpl.id);
                result.push(KbTemplate{
                    id: tpl.id, title: tpl.title.clone(), text: tpl.text.clone(),
                    keywords: tpl.keywords.clone(), category: tpl.category.clone(),
                    match_type: s("hotword"),
                });
                break;
            }
        }
    }

    if !result.is_empty() {
        if result.len() > 5 {
            return result.iter().take(5).collect::<Vec<KbTemplate>>();
        }
        return result;
    }

    let categories: Vec<(String, Vec<String>)> = vec![
        (s("售后退换"), vec![s("售后"), s("退换"), s("退款"), s("退货")]),
        (s("物流咨询"), vec![s("物流"), s("快递"), s("派送"), s("追踪"), s("发货")]),
        (s("售后投诉"), vec![s("售后"), s("投诉"), s("不满"), s("不满意")]),
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
                    match_type: s("category"),
                });
            }
        }
    }

    if result.len() > 5 {
        return result.iter().take(5).collect::<Vec<KbTemplate>>();
    }
    result
}
#[derive(Debug,Default,Clone)]
pub struct Config {
    /// 选择词库
    /// 0: "默认", 1: "诗经", 2: "楚辞", 3: "论语",
    /// 4: "周易", 5: "唐诗", 6: "宋诗", 7: "宋词"
    pub name_source: Vec<i64>,
    /// 姓，仅支持单姓
    pub last_name: String,
    /// 不想要的字，结果中不会出现这些字
    pub dislike_words: Vec<String>,
    /// 最小笔画数
    pub min_stroke_count: i64,
    /// 最大笔画数
    pub max_stroke_count: i64,
    /// 允许使用中吉，开启后将生成包含中吉配置的名字，生成的名字会更多
    pub allow_general: bool,
    /// 是否筛选名字，仅输出名字库中存在的名字，可以过滤明显不合适的名字
    pub name_validate: bool,
    /// 是否筛选性别，男/女，空则不筛选，仅当开启名字筛选时有效
    pub gender: String,
    /// 填入姓名，查看三才五格配置，仅支持单姓复名
    /// 如果要起名，请保持该值为空
    pub check_name: String,
    /// 是否显示名字来源
    pub check_name_resource: bool
}
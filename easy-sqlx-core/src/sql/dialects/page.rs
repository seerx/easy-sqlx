#[derive(Clone, Debug)]
pub enum OrderType {
    Asc,
    Desc,
    None,
}

impl OrderType {
    pub fn sql(&self) -> String {
        match self {
            OrderType::Asc => "asc".to_string(),
            OrderType::Desc => "desc".to_string(),
            OrderType::None => "".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Order {
    pub field: String,
    pub order_type: OrderType,
}

impl Order {
    /// 字段升序
    pub fn asc(field: String) -> Self {
        Self {
            field,
            order_type: OrderType::Asc,
        }
    }
    /// 字段降序
    pub fn desc(field: String) -> Self {
        Self {
            field,
            order_type: OrderType::Desc,
        }
    }
    /// 字段默认顺序
    pub fn new(field: String) -> Self {
        Self {
            field,
            order_type: OrderType::None,
        }
    }
}

pub struct PageRequest {
    /// 每页记录数
    page_size: usize,
    /// 页码，从 1 开始
    page_no: usize,
    /// 是否统计页面信息: 总记录数，总页数
    pub total_page_info: bool,
}

impl PageRequest {
    pub fn new(page_size: usize, page_no: usize) -> Self {
        Self {
            page_no,
            page_size,
            total_page_info: false,
        }
    }

    pub fn with_total(page_size: usize, page_no: usize) -> Self {
        Self {
            page_no,
            page_size,
            total_page_info: true,
        }
    }

    pub fn get_page_no(&self) -> usize {
        if self.page_no <= 0 {
            1
        } else {
            self.page_no
        }
    }

    pub fn get_page_size(&self) -> usize {
        if self.page_size <= 0 {
            20
        } else {
            self.page_size
        }
    }
}

#[derive(Default)]
pub struct PageResult<O>
where
    O: std::marker::Send,
    O: Unpin,
{
    /// 每页记录数
    pub page_size: usize,
    /// 页码，从 1 开始
    pub page_no: usize,
    /// 总记录数
    pub total: usize,
    /// 总页数
    pub page_count: usize,
    /// 记录
    pub records: Vec<O>,
}

impl<O> PageResult<O>
where
    O: std::marker::Send,
    O: Unpin,
{
    /// 设置总记录数，同时计算总页数
    pub fn set_total(&mut self, total: usize) {
        self.total = total;
        if total == 0 {
            self.page_count = 0;
        } else {
            let page_count = total / self.page_size;
            if total % self.page_size != 0 {
                self.page_count = page_count + 1;
            } else {
                self.page_count = page_count
            }
        }
    }
}

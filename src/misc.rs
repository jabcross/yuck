
use std::iter::Iterator;

pub trait NeatTreePrint {
    fn get_label(&self) -> String;

    fn get_children(&self) -> Vec<&dyn NeatTreePrint>;

    fn get_min_size(&self) -> usize {
        let children = self.get_children();
        let mut children_len : usize = 0;
        for child in children {
            children_len += child.get_min_size() + 1;
        }
        if children_len > 0 {
            children_len -= 1;
        }
        self.get_label().len().max(children_len)
    }

    fn as_line_vec(&self, min_width: Option<usize>) -> Vec<String> {
        let own_min_size = self.get_min_size();
        let (real_width, extra_padding) = if let Some(width) = min_width {
            if width >= own_min_size {
                (width, width - own_min_size)
            }
            else {
                (own_min_size, 0)
            }
        } 
        else {
            (own_min_size, 0)
        };

        let label = self.get_label();

        let left_padding = (real_width - label.len()) / 2;
        let right_padding = real_width - label.len() - left_padding;
        let mut rv = Vec::new();
        rv.push(format!("{space:left_padding$}{label}{space:right_padding$}",
            left_padding = left_padding,
            right_padding = right_padding,
            space = "",
            label = label,
            ));
        
        let children = self.get_children();

        if children.len() > 0 {

            let division = extra_padding / children.len();
            let rest = extra_padding % children.len();

            let mut children_line_lists = Vec::new();
            let mut max_depth = 0;

            for (index, child) in children.iter().enumerate() {
                let mut min_width = child.get_min_size() + 
                                division +
                                if index < rest {1} else {0};
                if children.len() == 1 {
                    min_width = min_width.max(label.len());
                }
                let line_list = child.as_line_vec(Some(min_width));
                max_depth = max_depth.max(line_list.len());
                children_line_lists.push(line_list);
            }

            for list in &mut children_line_lists {
                while list.len() < max_depth {
                    list.push(format!("{0:1$}","",list[0].len()));
                }
            }

            for index in 0..max_depth {
                let mut line = String::new();
                for list in &mut children_line_lists {
                    line += list[index].as_str();
                    line += "|";
                }
                line.pop();
                rv.push(line);
            }

        }

        rv
    }

    fn as_string(&self, min_width: Option<usize>) -> String {
        let mut rv = String::new();
        for line in self.as_line_vec(min_width) {
            rv += "|";
            rv += line.as_str();
            rv += "|\n";
        }
        rv
    }

}

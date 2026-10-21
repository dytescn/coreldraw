//! 枚举接口离线测试
//!
//! 不依赖 CorelDRAW：验证关键枚举的真实整数值、`from_i32` 往返与重复值别名。

use cdrsdk::enums::*;

#[test]
fn enums_key_values_and_roundtrip() {
    // 图形类型
    assert_eq!(cdrShapeType::RectangleShape as i32, 1);
    assert_eq!(cdrShapeType::TextShape as i32, 6);
    assert_eq!(cdrShapeType::from_i32(6), Some(cdrShapeType::TextShape));
    assert_eq!(cdrShapeType::from_i32(9999), None);

    // 文件过滤器 + 重复值别名
    assert_eq!(cdrFilter::PNG as i32, 802);
    assert_eq!(cdrFilter::from_i32(802), Some(cdrFilter::PNG));
    assert_eq!(cdrFilter::PDF, cdrFilter::AI9);

    // 单位
    assert_eq!(cdrUnit::Inch as i32, 1);
    assert_eq!(cdrUnit::Millimeter as i32, 3);
    assert_eq!(cdrUnit::Point as i32, 14);
    assert_eq!(cdrUnit::from_i32(1), Some(cdrUnit::Inch));
    assert_eq!(cdrUnit::from_i32(14), Some(cdrUnit::Point));

    // 文本
    assert_eq!(cdrTextType::Artistic as i32, 0);
    assert_eq!(cdrTextType::from_i32(1), Some(cdrTextType::Paragraph));
    assert_eq!(cdrAlignment::from_i32(1), Some(cdrAlignment::LeftAlignment));

    // 填充 / 轮廓 / 效果 / 颜色
    assert_eq!(cdrFillType::from_i32(2), Some(cdrFillType::FountainFill));
    assert_eq!(cdrOutlineType::from_i32(0), Some(cdrOutlineType::NoOutline));
    assert_eq!(cdrEffectType::from_i32(5), Some(cdrEffectType::DropShadow));
    assert_eq!(cdrColorType::from_i32(5), Some(cdrColorType::RGB));

    // 三态布尔
    assert_eq!(cdrTriState::from_i32(-1), Some(cdrTriState::True));
    assert_eq!(cdrTriState::True.to_bool(), Some(true));
    assert_eq!(cdrTriState::Undefined.to_bool(), None);

    // 应用 ID
    assert_eq!(cdrApplicationID::from_i32(0), Some(cdrApplicationID::CorelDRAW));
}

#[test]
fn enums_fill_and_outline_values() {
    assert_eq!(cdrFountainFillType::LinearFountainFill as i32, 1);
    assert_eq!(cdrFountainFillBlendType::DirectFountainFillBlend as i32, 0);
    assert_eq!(cdrOutlineLineCaps::Round as i32, 1);
    assert_eq!(cdrOutlineLineCaps::Square as i32, 2);
    assert_eq!(cdrMergeMode::MergeNormal as i32, 0);
    assert_eq!(cdrDropShadowType::DropShadowFlat as i32, 0);
    assert_eq!(cdrContourDirection::Outside as i32, 1);
}

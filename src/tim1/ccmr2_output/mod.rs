#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCMR2_OUTPUT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct OC4CER {
    bits: bool,
}
impl OC4CER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `OC4M`"]
pub type OC4MR = super::ccmr1_output::OC1MR;
#[doc = r" Value of the field"]
pub struct OC4PER {
    bits: bool,
}
impl OC4PER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct OC4FER {
    bits: bool,
}
impl OC4FER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CC4SR {
    bits: u8,
}
impl CC4SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OC3CER {
    bits: bool,
}
impl OC3CER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `OC3M`"]
pub type OC3MR = super::ccmr1_output::OC1MR;
#[doc = r" Value of the field"]
pub struct OC3PER {
    bits: bool,
}
impl OC3PER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct OC3FER {
    bits: bool,
}
impl OC3FER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CC3SR {
    bits: u8,
}
impl CC3SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _OC4CEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC4CEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OC4M`"]
pub type OC4MW = super::ccmr1_output::OC1MW;
#[doc = r" Proxy"]
pub struct _OC4MW<'a> {
    w: &'a mut W,
}
impl<'a> _OC4MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC4MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs("]
    #[inline]
    pub fn frozen(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FROZEN)
    }
    #[doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_active(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::SETACTIVE)
    }
    #[doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::SETINACTIVE)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::TOGGLE)
    }
    #[doc = "OCyREF is forced low."]
    #[inline]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FORCEINACTIVE)
    }
    #[doc = "OCyREF is forced high."]
    #[inline]
    pub fn force_active(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel 1 is inactive (OCyREF=\u{2018}0) as long as TIMx_CNT>TIMx_CCRy else active (OCyREF=1)."]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::PWM1)
    }
    #[doc = "In upcounting, channel y is inactive as long as TIMx_CNT<TIMx_CCRy else active. In downcounting, channel y is active as long as TIMx_CNT>TIMx_CCRy else inactive."]
    #[inline]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::PWM2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OC4PEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC4PEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OC4FEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC4FEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CC4SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC4SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OC3CEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC3CEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OC3M`"]
pub type OC3MW = super::ccmr1_output::OC1MW;
#[doc = r" Proxy"]
pub struct _OC3MW<'a> {
    w: &'a mut W,
}
impl<'a> _OC3MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC3MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs("]
    #[inline]
    pub fn frozen(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FROZEN)
    }
    #[doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_active(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::SETACTIVE)
    }
    #[doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::SETINACTIVE)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::TOGGLE)
    }
    #[doc = "OCyREF is forced low."]
    #[inline]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FORCEINACTIVE)
    }
    #[doc = "OCyREF is forced high."]
    #[inline]
    pub fn force_active(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel 1 is inactive (OCyREF=\u{2018}0) as long as TIMx_CNT>TIMx_CCRy else active (OCyREF=1)."]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::PWM1)
    }
    #[doc = "In upcounting, channel y is inactive as long as TIMx_CNT<TIMx_CCRy else active. In downcounting, channel y is active as long as TIMx_CNT>TIMx_CCRy else inactive."]
    #[inline]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(super::ccmr1_output::OC1MW::PWM2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OC3PEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC3PEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OC3FEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC3FEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CC3SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC3SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline]
    pub fn oc4ce(&self) -> OC4CER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OC4CER { bits }
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline]
    pub fn oc4m(&self) -> OC4MR {
        OC4MR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline]
    pub fn oc4pe(&self) -> OC4PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OC4PER { bits }
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline]
    pub fn oc4fe(&self) -> OC4FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OC4FER { bits }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline]
    pub fn cc4s(&self) -> CC4SR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CC4SR { bits }
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline]
    pub fn oc3ce(&self) -> OC3CER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OC3CER { bits }
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline]
    pub fn oc3m(&self) -> OC3MR {
        OC3MR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline]
    pub fn oc3pe(&self) -> OC3PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OC3PER { bits }
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline]
    pub fn oc3fe(&self) -> OC3FER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OC3FER { bits }
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline]
    pub fn cc3s(&self) -> CC3SR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CC3SR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline]
    pub fn oc4ce(&mut self) -> _OC4CEW {
        _OC4CEW { w: self }
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline]
    pub fn oc4m(&mut self) -> _OC4MW {
        _OC4MW { w: self }
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline]
    pub fn oc4pe(&mut self) -> _OC4PEW {
        _OC4PEW { w: self }
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline]
    pub fn oc4fe(&mut self) -> _OC4FEW {
        _OC4FEW { w: self }
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline]
    pub fn cc4s(&mut self) -> _CC4SW {
        _CC4SW { w: self }
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline]
    pub fn oc3ce(&mut self) -> _OC3CEW {
        _OC3CEW { w: self }
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline]
    pub fn oc3m(&mut self) -> _OC3MW {
        _OC3MW { w: self }
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline]
    pub fn oc3pe(&mut self) -> _OC3PEW {
        _OC3PEW { w: self }
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline]
    pub fn oc3fe(&mut self) -> _OC3FEW {
        _OC3FEW { w: self }
    }
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline]
    pub fn cc3s(&mut self) -> _CC3SW {
        _CC3SW { w: self }
    }
}

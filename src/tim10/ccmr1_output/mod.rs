#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCMR1_OUTPUT {
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
#[doc = "Possible values of the field `OC1M`"]
pub type OC1MR = ::tim1::ccmr1_output::OC1MR;
#[doc = r" Value of the field"]
pub struct OC1PER {
    bits: bool,
}
impl OC1PER {
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
pub struct CC1SR {
    bits: u8,
}
impl CC1SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `OC1M`"]
pub type OC1MW = ::tim1::ccmr1_output::OC1MW;
#[doc = r" Proxy"]
pub struct _OC1MW<'a> {
    w: &'a mut W,
}
impl<'a> _OC1MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OC1MW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs("]
    #[inline]
    pub fn frozen(self) -> &'a mut W {
        self.variant(::tim1::ccmr1_output::OC1MW::FROZEN)
    }
    #[doc = "Set channel y to active level on match. OCyREF signal is forced high when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_active(self) -> &'a mut W {
        self.variant(::tim1::ccmr1_output::OC1MW::SETACTIVE)
    }
    #[doc = "Set channel y to inactive level on match. OCyREF signal is forced low when the counter TIMx_CNT matches the capture/compare register y (TIMx_CCRy)."]
    #[inline]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(::tim1::ccmr1_output::OC1MW::SETINACTIVE)
    }
    #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy."]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(::tim1::ccmr1_output::OC1MW::TOGGLE)
    }
    #[doc = "OCyREF is forced low."]
    #[inline]
    pub fn force_inactive(self) -> &'a mut W {
        self.variant(::tim1::ccmr1_output::OC1MW::FORCEINACTIVE)
    }
    #[doc = "OCyREF is forced high."]
    #[inline]
    pub fn force_active(self) -> &'a mut W {
        self.variant(::tim1::ccmr1_output::OC1MW::FORCEACTIVE)
    }
    #[doc = "In upcounting, channel 1 is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel 1 is inactive (OCyREF=\u{2018}0) as long as TIMx_CNT>TIMx_CCRy else active (OCyREF=1)."]
    #[inline]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(::tim1::ccmr1_output::OC1MW::PWM1)
    }
    #[doc = "In upcounting, channel y is inactive as long as TIMx_CNT<TIMx_CCRy else active. In downcounting, channel y is active as long as TIMx_CNT>TIMx_CCRy else inactive."]
    #[inline]
    pub fn pwm2(self) -> &'a mut W {
        self.variant(::tim1::ccmr1_output::OC1MW::PWM2)
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
pub struct _OC1PEW<'a> {
    w: &'a mut W,
}
impl<'a> _OC1PEW<'a> {
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
pub struct _CC1SW<'a> {
    w: &'a mut W,
}
impl<'a> _CC1SW<'a> {
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
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline]
    pub fn oc1m(&self) -> OC1MR {
        OC1MR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline]
    pub fn oc1pe(&self) -> OC1PER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OC1PER { bits }
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline]
    pub fn cc1s(&self) -> CC1SR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CC1SR { bits }
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
    #[doc = "Bits 4:6 - Output Compare 1 mode"]
    #[inline]
    pub fn oc1m(&mut self) -> _OC1MW {
        _OC1MW { w: self }
    }
    #[doc = "Bit 3 - Output Compare 1 preload enable"]
    #[inline]
    pub fn oc1pe(&mut self) -> _OC1PEW {
        _OC1PEW { w: self }
    }
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline]
    pub fn cc1s(&mut self) -> _CC1SW {
        _CC1SW { w: self }
    }
}

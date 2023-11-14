#[doc = "Register `TXMDTR1` reader"]
pub type R = crate::R<TXMDTR1_SPEC>;
#[doc = "Register `TXMDTR1` writer"]
pub type W = crate::W<TXMDTR1_SPEC>;
#[doc = "Field `DLC` reader - Data length code"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - Data length code"]
pub type DLC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TGT` reader - Transmit global time"]
pub type TGT_R = crate::BitReader;
#[doc = "Field `TGT` writer - Transmit global time"]
pub type TGT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIME` reader - Message time stamp"]
pub type TIME_R = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - Message time stamp"]
pub type TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Transmit global time"]
    #[inline(always)]
    pub fn tgt(&self) -> TGT_R {
        TGT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Message time stamp"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<TXMDTR1_SPEC, 0> {
        DLC_W::new(self)
    }
    #[doc = "Bit 8 - Transmit global time"]
    #[inline(always)]
    #[must_use]
    pub fn tgt(&mut self) -> TGT_W<TXMDTR1_SPEC, 8> {
        TGT_W::new(self)
    }
    #[doc = "Bits 16:31 - Message time stamp"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<TXMDTR1_SPEC, 16> {
        TIME_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CAN mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmdtr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmdtr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMDTR1_SPEC;
impl crate::RegisterSpec for TXMDTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmdtr1::R`](R) reader structure"]
impl crate::Readable for TXMDTR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txmdtr1::W`](W) writer structure"]
impl crate::Writable for TXMDTR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXMDTR1 to value 0"]
impl crate::Resettable for TXMDTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

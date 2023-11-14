#[doc = "Register `RFIFO1` reader"]
pub type R = crate::R<RFIFO1_SPEC>;
#[doc = "Register `RFIFO1` writer"]
pub type W = crate::W<RFIFO1_SPEC>;
#[doc = "Field `FMP1` reader - FIFO 1 message pending"]
pub type FMP1_R = crate::FieldReader;
#[doc = "Field `FULL1` reader - FIFO 1 full"]
pub type FULL1_R = crate::BitReader;
#[doc = "Field `FULL1` writer - FIFO 1 full"]
pub type FULL1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FOVR1` reader - FIFO 1 overrun"]
pub type FOVR1_R = crate::BitReader;
#[doc = "Field `FOVR1` writer - FIFO 1 overrun"]
pub type FOVR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFOM1` reader - Release FIFO 1 output mailbox"]
pub type RFOM1_R = crate::BitReader;
#[doc = "Field `RFOM1` writer - Release FIFO 1 output mailbox"]
pub type RFOM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - FIFO 1 message pending"]
    #[inline(always)]
    pub fn fmp1(&self) -> FMP1_R {
        FMP1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FIFO 1 full"]
    #[inline(always)]
    pub fn full1(&self) -> FULL1_R {
        FULL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO 1 overrun"]
    #[inline(always)]
    pub fn fovr1(&self) -> FOVR1_R {
        FOVR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Release FIFO 1 output mailbox"]
    #[inline(always)]
    pub fn rfom1(&self) -> RFOM1_R {
        RFOM1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FIFO 1 full"]
    #[inline(always)]
    #[must_use]
    pub fn full1(&mut self) -> FULL1_W<RFIFO1_SPEC, 3> {
        FULL1_W::new(self)
    }
    #[doc = "Bit 4 - FIFO 1 overrun"]
    #[inline(always)]
    #[must_use]
    pub fn fovr1(&mut self) -> FOVR1_W<RFIFO1_SPEC, 4> {
        FOVR1_W::new(self)
    }
    #[doc = "Bit 5 - Release FIFO 1 output mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn rfom1(&mut self) -> RFOM1_W<RFIFO1_SPEC, 5> {
        RFOM1_W::new(self)
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
#[doc = "CAN receive FIFO 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIFO1_SPEC;
impl crate::RegisterSpec for RFIFO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo1::R`](R) reader structure"]
impl crate::Readable for RFIFO1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfifo1::W`](W) writer structure"]
impl crate::Writable for RFIFO1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIFO1 to value 0"]
impl crate::Resettable for RFIFO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

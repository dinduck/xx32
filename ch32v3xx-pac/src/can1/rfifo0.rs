#[doc = "Register `RFIFO0` reader"]
pub type R = crate::R<RFIFO0_SPEC>;
#[doc = "Register `RFIFO0` writer"]
pub type W = crate::W<RFIFO0_SPEC>;
#[doc = "Field `FMP0` reader - FIFO 0 message pending"]
pub type FMP0_R = crate::FieldReader;
#[doc = "Field `FULL0` reader - FIFO 0 full"]
pub type FULL0_R = crate::BitReader;
#[doc = "Field `FULL0` writer - FIFO 0 full"]
pub type FULL0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FOVR0` reader - FIFO 0 overrun"]
pub type FOVR0_R = crate::BitReader;
#[doc = "Field `FOVR0` writer - FIFO 0 overrun"]
pub type FOVR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RFOM0` reader - Release FIFO 0 output mailbox"]
pub type RFOM0_R = crate::BitReader;
#[doc = "Field `RFOM0` writer - Release FIFO 0 output mailbox"]
pub type RFOM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - FIFO 0 message pending"]
    #[inline(always)]
    pub fn fmp0(&self) -> FMP0_R {
        FMP0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FIFO 0 full"]
    #[inline(always)]
    pub fn full0(&self) -> FULL0_R {
        FULL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO 0 overrun"]
    #[inline(always)]
    pub fn fovr0(&self) -> FOVR0_R {
        FOVR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Release FIFO 0 output mailbox"]
    #[inline(always)]
    pub fn rfom0(&self) -> RFOM0_R {
        RFOM0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FIFO 0 full"]
    #[inline(always)]
    #[must_use]
    pub fn full0(&mut self) -> FULL0_W<RFIFO0_SPEC, 3> {
        FULL0_W::new(self)
    }
    #[doc = "Bit 4 - FIFO 0 overrun"]
    #[inline(always)]
    #[must_use]
    pub fn fovr0(&mut self) -> FOVR0_W<RFIFO0_SPEC, 4> {
        FOVR0_W::new(self)
    }
    #[doc = "Bit 5 - Release FIFO 0 output mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn rfom0(&mut self) -> RFOM0_W<RFIFO0_SPEC, 5> {
        RFOM0_W::new(self)
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
#[doc = "CAN receive FIFO 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfifo0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfifo0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIFO0_SPEC;
impl crate::RegisterSpec for RFIFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfifo0::R`](R) reader structure"]
impl crate::Readable for RFIFO0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfifo0::W`](W) writer structure"]
impl crate::Writable for RFIFO0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIFO0 to value 0"]
impl crate::Resettable for RFIFO0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `TXMIR2` reader"]
pub type R = crate::R<TXMIR2_SPEC>;
#[doc = "Register `TXMIR2` writer"]
pub type W = crate::W<TXMIR2_SPEC>;
#[doc = "Field `TXRQ` reader - Transmit mailbox request"]
pub type TXRQ_R = crate::BitReader;
#[doc = "Field `TXRQ` writer - Transmit mailbox request"]
pub type TXRQ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTR` reader - Remote transmission request"]
pub type RTR_R = crate::BitReader;
#[doc = "Field `RTR` writer - Remote transmission request"]
pub type RTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IDE` reader - Identifier extension"]
pub type IDE_R = crate::BitReader;
#[doc = "Field `IDE` writer - Identifier extension"]
pub type IDE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXID` reader - extended identifier"]
pub type EXID_R = crate::FieldReader<u32>;
#[doc = "Field `EXID` writer - extended identifier"]
pub type EXID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 18, O, u32>;
#[doc = "Field `STID` reader - Standard identifier"]
pub type STID_R = crate::FieldReader<u16>;
#[doc = "Field `STID` writer - Standard identifier"]
pub type STID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl R {
    #[doc = "Bit 0 - Transmit mailbox request"]
    #[inline(always)]
    pub fn txrq(&self) -> TXRQ_R {
        TXRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remote transmission request"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Identifier extension"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - extended identifier"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - Standard identifier"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox request"]
    #[inline(always)]
    #[must_use]
    pub fn txrq(&mut self) -> TXRQ_W<TXMIR2_SPEC, 0> {
        TXRQ_W::new(self)
    }
    #[doc = "Bit 1 - Remote transmission request"]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<TXMIR2_SPEC, 1> {
        RTR_W::new(self)
    }
    #[doc = "Bit 2 - Identifier extension"]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<TXMIR2_SPEC, 2> {
        IDE_W::new(self)
    }
    #[doc = "Bits 3:20 - extended identifier"]
    #[inline(always)]
    #[must_use]
    pub fn exid(&mut self) -> EXID_W<TXMIR2_SPEC, 3> {
        EXID_W::new(self)
    }
    #[doc = "Bits 21:31 - Standard identifier"]
    #[inline(always)]
    #[must_use]
    pub fn stid(&mut self) -> STID_W<TXMIR2_SPEC, 21> {
        STID_W::new(self)
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
#[doc = "CAN TX mailbox identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txmir2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txmir2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXMIR2_SPEC;
impl crate::RegisterSpec for TXMIR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmir2::R`](R) reader structure"]
impl crate::Readable for TXMIR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txmir2::W`](W) writer structure"]
impl crate::Writable for TXMIR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXMIR2 to value 0"]
impl crate::Resettable for TXMIR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

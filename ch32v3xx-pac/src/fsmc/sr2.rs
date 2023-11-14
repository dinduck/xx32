#[doc = "Register `SR2` reader"]
pub type R = crate::R<SR2_SPEC>;
#[doc = "Register `SR2` writer"]
pub type W = crate::W<SR2_SPEC>;
#[doc = "Field `IRS` reader - Interrupt rising edge status"]
pub type IRS_R = crate::BitReader;
#[doc = "Field `IRS` writer - Interrupt rising edge status"]
pub type IRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILS` reader - Interrupt high-level status"]
pub type ILS_R = crate::BitReader;
#[doc = "Field `ILS` writer - Interrupt high-level status"]
pub type ILS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IFS` reader - Interrupt falling edge status"]
pub type IFS_R = crate::BitReader;
#[doc = "Field `IFS` writer - Interrupt falling edge status"]
pub type IFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IREN` reader - Interrupt rising edge detection enable bit"]
pub type IREN_R = crate::BitReader;
#[doc = "Field `IREN` writer - Interrupt rising edge detection enable bit"]
pub type IREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ILEN` reader - Interrupt high-level detection enable bit"]
pub type ILEN_R = crate::BitReader;
#[doc = "Field `ILEN` writer - Interrupt high-level detection enable bit"]
pub type ILEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IFEN` reader - Interrupt falling edge detection enable bit"]
pub type IFEN_R = crate::BitReader;
#[doc = "Field `IFEN` writer - Interrupt falling edge detection enable bit"]
pub type IFEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FEMPT` reader - FIFO empty"]
pub type FEMPT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    pub fn irs(&self) -> IRS_R {
        IRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    pub fn ils(&self) -> ILS_R {
        ILS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    pub fn ilen(&self) -> ILEN_R {
        ILEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    pub fn ifen(&self) -> IFEN_R {
        IFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO empty"]
    #[inline(always)]
    pub fn fempt(&self) -> FEMPT_R {
        FEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt rising edge status"]
    #[inline(always)]
    #[must_use]
    pub fn irs(&mut self) -> IRS_W<SR2_SPEC, 0> {
        IRS_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt high-level status"]
    #[inline(always)]
    #[must_use]
    pub fn ils(&mut self) -> ILS_W<SR2_SPEC, 1> {
        ILS_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt falling edge status"]
    #[inline(always)]
    #[must_use]
    pub fn ifs(&mut self) -> IFS_W<SR2_SPEC, 2> {
        IFS_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt rising edge detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<SR2_SPEC, 3> {
        IREN_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt high-level detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ilen(&mut self) -> ILEN_W<SR2_SPEC, 4> {
        ILEN_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt falling edge detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ifen(&mut self) -> IFEN_W<SR2_SPEC, 5> {
        IFEN_W::new(self)
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
#[doc = "FIFO status and interrupt register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for SR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr2::W`](W) writer structure"]
impl crate::Writable for SR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR2 to value 0x40"]
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}

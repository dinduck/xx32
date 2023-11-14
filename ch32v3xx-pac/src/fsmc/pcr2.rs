#[doc = "Register `PCR2` reader"]
pub type R = crate::R<PCR2_SPEC>;
#[doc = "Register `PCR2` writer"]
pub type W = crate::W<PCR2_SPEC>;
#[doc = "Field `PWAITEN` reader - Wait feature enable bit"]
pub type PWAITEN_R = crate::BitReader;
#[doc = "Field `PWAITEN` writer - Wait feature enable bit"]
pub type PWAITEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PBKEN` reader - PC Card/NAND Flash memory bank enable bit"]
pub type PBKEN_R = crate::BitReader;
#[doc = "Field `PBKEN` writer - PC Card/NAND Flash memory bank enable bit"]
pub type PBKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTYP` reader - Memory type"]
pub type PTYP_R = crate::BitReader;
#[doc = "Field `PTYP` writer - Memory type"]
pub type PTYP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWID` reader - Databus width"]
pub type PWID_R = crate::FieldReader;
#[doc = "Field `PWID` writer - Databus width"]
pub type PWID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ECCEN` reader - ECC computation logic enable bit"]
pub type ECCEN_R = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECC computation logic enable bit"]
pub type ECCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TCLR` reader - CLE to RE delay"]
pub type TCLR_R = crate::FieldReader;
#[doc = "Field `TCLR` writer - CLE to RE delay"]
pub type TCLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TAR` reader - ALE to RE delay"]
pub type TAR_R = crate::FieldReader;
#[doc = "Field `TAR` writer - ALE to RE delay"]
pub type TAR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ECCPS` reader - ECC page size"]
pub type ECCPS_R = crate::FieldReader;
#[doc = "Field `ECCPS` writer - ECC page size"]
pub type ECCPS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 1 - Wait feature enable bit"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PC Card/NAND Flash memory bank enable bit"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Memory type"]
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Databus width"]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECC computation logic enable bit"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECC page size"]
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwaiten(&mut self) -> PWAITEN_W<PCR2_SPEC, 1> {
        PWAITEN_W::new(self)
    }
    #[doc = "Bit 2 - PC Card/NAND Flash memory bank enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pbken(&mut self) -> PBKEN_W<PCR2_SPEC, 2> {
        PBKEN_W::new(self)
    }
    #[doc = "Bit 3 - Memory type"]
    #[inline(always)]
    #[must_use]
    pub fn ptyp(&mut self) -> PTYP_W<PCR2_SPEC, 3> {
        PTYP_W::new(self)
    }
    #[doc = "Bits 4:5 - Databus width"]
    #[inline(always)]
    #[must_use]
    pub fn pwid(&mut self) -> PWID_W<PCR2_SPEC, 4> {
        PWID_W::new(self)
    }
    #[doc = "Bit 6 - ECC computation logic enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<PCR2_SPEC, 6> {
        ECCEN_W::new(self)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<PCR2_SPEC, 9> {
        TCLR_W::new(self)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<PCR2_SPEC, 13> {
        TAR_W::new(self)
    }
    #[doc = "Bits 17:19 - ECC page size"]
    #[inline(always)]
    #[must_use]
    pub fn eccps(&mut self) -> ECCPS_W<PCR2_SPEC, 17> {
        ECCPS_W::new(self)
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
#[doc = "PC Card/NAND Flash control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCR2_SPEC;
impl crate::RegisterSpec for PCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr2::R`](R) reader structure"]
impl crate::Readable for PCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcr2::W`](W) writer structure"]
impl crate::Writable for PCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCR2 to value 0x18"]
impl crate::Resettable for PCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x18;
}

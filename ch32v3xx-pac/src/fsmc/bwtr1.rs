#[doc = "Register `BWTR1` reader"]
pub type R = crate::R<BWTR1_SPEC>;
#[doc = "Register `BWTR1` writer"]
pub type W = crate::W<BWTR1_SPEC>;
#[doc = "Field `ADDSET` reader - Address setup phase duration"]
pub type ADDSET_R = crate::FieldReader;
#[doc = "Field `ADDSET` writer - Address setup phase duration"]
pub type ADDSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ADDHLD` reader - Address-hold phase duration"]
pub type ADDHLD_R = crate::FieldReader;
#[doc = "Field `ADDHLD` writer - Address-hold phase duration"]
pub type ADDHLD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DATAST` reader - Data-phase duration"]
pub type DATAST_R = crate::FieldReader;
#[doc = "Field `DATAST` writer - Data-phase duration"]
pub type DATAST_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `CLKDIV` reader - Clock divide ratio (for FSMC_CLK signal)"]
pub type CLKDIV_R = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock divide ratio (for FSMC_CLK signal)"]
pub type CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DATLAT` reader - Data latency for synchronous NOR Flash memory"]
pub type DATLAT_R = crate::FieldReader;
#[doc = "Field `DATLAT` writer - Data latency for synchronous NOR Flash memory"]
pub type DATLAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `ACCMOD` reader - Access mode"]
pub type ACCMOD_R = crate::FieldReader;
#[doc = "Field `ACCMOD` writer - Access mode"]
pub type ACCMOD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Address setup phase duration"]
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration"]
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Data-phase duration"]
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FSMC_CLK signal)"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Data latency for synchronous NOR Flash memory"]
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Access mode"]
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<BWTR1_SPEC, 0> {
        ADDSET_W::new(self)
    }
    #[doc = "Bits 4:7 - Address-hold phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<BWTR1_SPEC, 4> {
        ADDHLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Data-phase duration"]
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<BWTR1_SPEC, 8> {
        DATAST_W::new(self)
    }
    #[doc = "Bits 20:23 - Clock divide ratio (for FSMC_CLK signal)"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<BWTR1_SPEC, 20> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 24:27 - Data latency for synchronous NOR Flash memory"]
    #[inline(always)]
    #[must_use]
    pub fn datlat(&mut self) -> DATLAT_W<BWTR1_SPEC, 24> {
        DATLAT_W::new(self)
    }
    #[doc = "Bits 28:29 - Access mode"]
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<BWTR1_SPEC, 28> {
        ACCMOD_W::new(self)
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
#[doc = "SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bwtr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bwtr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BWTR1_SPEC;
impl crate::RegisterSpec for BWTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bwtr1::R`](R) reader structure"]
impl crate::Readable for BWTR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bwtr1::W`](W) writer structure"]
impl crate::Writable for BWTR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BWTR1 to value 0x0fff_ffff"]
impl crate::Resettable for BWTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}

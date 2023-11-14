#[doc = "Register `SPI_I2S_CFGR` reader"]
pub type R = crate::R<SPI_I2S_CFGR_SPEC>;
#[doc = "Register `SPI_I2S_CFGR` writer"]
pub type W = crate::W<SPI_I2S_CFGR_SPEC>;
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel)"]
pub type CHLEN_R = crate::BitReader;
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel)"]
pub type CHLEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DATLEN` reader - DATLEN\\[1:0\\]
bits (Data length to be transferred)"]
pub type DATLEN_R = crate::FieldReader;
#[doc = "Field `DATLEN` writer - DATLEN\\[1:0\\]
bits (Data length to be transferred)"]
pub type DATLEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CKPOL` reader - steady state clock polarity"]
pub type CKPOL_R = crate::BitReader;
#[doc = "Field `CKPOL` writer - steady state clock polarity"]
pub type CKPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2SSTD` reader - I2SSTD\\[1:0\\]
bits (I2S standard selection)"]
pub type I2SSTD_R = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - I2SSTD\\[1:0\\]
bits (I2S standard selection)"]
pub type I2SSTD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization"]
pub type PCMSYNC_R = crate::BitReader;
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization"]
pub type PCMSYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2SCFG` reader - I2SCFG\\[1:0\\]
bits (I2S configuration mode)"]
pub type I2SCFG_R = crate::FieldReader;
#[doc = "Field `I2SCFG` writer - I2SCFG\\[1:0\\]
bits (I2S configuration mode)"]
pub type I2SCFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `I2SE` reader - I2S Enable"]
pub type I2SE_R = crate::BitReader;
#[doc = "Field `I2SE` writer - I2S Enable"]
pub type I2SE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2SMOD` reader - I2S mode selection"]
pub type I2SMOD_R = crate::BitReader;
#[doc = "Field `I2SMOD` writer - I2S mode selection"]
pub type I2SMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - DATLEN\\[1:0\\]
bits (Data length to be transferred)"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - steady state clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2SSTD\\[1:0\\]
bits (I2S standard selection)"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2SCFG\\[1:0\\]
bits (I2S configuration mode)"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<SPI_I2S_CFGR_SPEC, 0> {
        CHLEN_W::new(self)
    }
    #[doc = "Bits 1:2 - DATLEN\\[1:0\\]
bits (Data length to be transferred)"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<SPI_I2S_CFGR_SPEC, 1> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 3 - steady state clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<SPI_I2S_CFGR_SPEC, 3> {
        CKPOL_W::new(self)
    }
    #[doc = "Bits 4:5 - I2SSTD\\[1:0\\]
bits (I2S standard selection)"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<SPI_I2S_CFGR_SPEC, 4> {
        I2SSTD_W::new(self)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<SPI_I2S_CFGR_SPEC, 7> {
        PCMSYNC_W::new(self)
    }
    #[doc = "Bits 8:9 - I2SCFG\\[1:0\\]
bits (I2S configuration mode)"]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<SPI_I2S_CFGR_SPEC, 8> {
        I2SCFG_W::new(self)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2SE_W<SPI_I2S_CFGR_SPEC, 10> {
        I2SE_W::new(self)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<SPI_I2S_CFGR_SPEC, 11> {
        I2SMOD_W::new(self)
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
#[doc = "SPI_I2S configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_i2s_cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_i2s_cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_I2S_CFGR_SPEC;
impl crate::RegisterSpec for SPI_I2S_CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_i2s_cfgr::R`](R) reader structure"]
impl crate::Readable for SPI_I2S_CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_i2s_cfgr::W`](W) writer structure"]
impl crate::Writable for SPI_I2S_CFGR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_I2S_CFGR to value 0"]
impl crate::Resettable for SPI_I2S_CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `OBR` reader"]
pub type R = crate::R<OBR_SPEC>;
#[doc = "Field `OBERR` reader - Option byte error"]
pub type OBERR_R = crate::BitReader;
#[doc = "Field `RDPRT` reader - Read protection"]
pub type RDPRT_R = crate::BitReader;
#[doc = "Field `IWDG_SW` reader - IWDG_SW"]
pub type IWDG_SW_R = crate::BitReader;
#[doc = "Field `STOP_RST` reader - STOP_RST"]
pub type STOP_RST_R = crate::BitReader;
#[doc = "Field `STANDY_RST` reader - STANDY_RST"]
pub type STANDY_RST_R = crate::BitReader;
#[doc = "Field `SRAM_CODE_MODE` reader - SRAM_CODE_MODE"]
pub type SRAM_CODE_MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn oberr(&self) -> OBERR_R {
        OBERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read protection"]
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IWDG_SW"]
    #[inline(always)]
    pub fn iwdg_sw(&self) -> IWDG_SW_R {
        IWDG_SW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP_RST"]
    #[inline(always)]
    pub fn stop_rst(&self) -> STOP_RST_R {
        STOP_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STANDY_RST"]
    #[inline(always)]
    pub fn standy_rst(&self) -> STANDY_RST_R {
        STANDY_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - SRAM_CODE_MODE"]
    #[inline(always)]
    pub fn sram_code_mode(&self) -> SRAM_CODE_MODE_R {
        SRAM_CODE_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "Option byte register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`obr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OBR_SPEC;
impl crate::RegisterSpec for OBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obr::R`](R) reader structure"]
impl crate::Readable for OBR_SPEC {}
#[doc = "`reset()` method sets OBR to value 0x03ff_fffc"]
impl crate::Resettable for OBR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff_fffc;
}

#[doc = "Register `RXMDTR0` reader"]
pub type R = crate::R<RXMDTR0_SPEC>;
#[doc = "Field `DLC` reader - Data length code"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `FMI` reader - Filter match index"]
pub type FMI_R = crate::FieldReader;
#[doc = "Field `TIME` reader - Message time stamp"]
pub type TIME_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Data length code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Filter match index"]
    #[inline(always)]
    pub fn fmi(&self) -> FMI_R {
        FMI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Message time stamp"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "CAN receive FIFO mailbox data length control and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmdtr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMDTR0_SPEC;
impl crate::RegisterSpec for RXMDTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmdtr0::R`](R) reader structure"]
impl crate::Readable for RXMDTR0_SPEC {}
#[doc = "`reset()` method sets RXMDTR0 to value 0"]
impl crate::Resettable for RXMDTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `RXMDLR0` reader"]
pub type R = crate::R<RXMDLR0_SPEC>;
#[doc = "Field `DATA0` reader - Data Byte 0"]
pub type DATA0_R = crate::FieldReader;
#[doc = "Field `DATA1` reader - Data Byte 1"]
pub type DATA1_R = crate::FieldReader;
#[doc = "Field `DATA2` reader - Data Byte 2"]
pub type DATA2_R = crate::FieldReader;
#[doc = "Field `DATA3` reader - Data Byte 3"]
pub type DATA3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Data Byte 0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data Byte 1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Byte 2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data Byte 3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CAN receive FIFO mailbox data low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmdlr0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMDLR0_SPEC;
impl crate::RegisterSpec for RXMDLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmdlr0::R`](R) reader structure"]
impl crate::Readable for RXMDLR0_SPEC {}
#[doc = "`reset()` method sets RXMDLR0 to value 0"]
impl crate::Resettable for RXMDLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `BCR1` reader"]
pub type R = crate::R<BCR1_SPEC>;
#[doc = "Register `BCR1` writer"]
pub type W = crate::W<BCR1_SPEC>;
#[doc = "Field `MBKEN` reader - Memory bank enable bit"]
pub type MBKEN_R = crate::BitReader;
#[doc = "Field `MBKEN` writer - Memory bank enable bit"]
pub type MBKEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MUXEN` reader - Address/data multiplexing enable bit"]
pub type MUXEN_R = crate::BitReader;
#[doc = "Field `MUXEN` writer - Address/data multiplexing enable bit"]
pub type MUXEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTYP` reader - Memory type"]
pub type MTYP_R = crate::FieldReader;
#[doc = "Field `MTYP` writer - Memory type"]
pub type MTYP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MWID` reader - Memory databus width"]
pub type MWID_R = crate::FieldReader;
#[doc = "Field `MWID` writer - Memory databus width"]
pub type MWID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FACCEN` reader - Flash access enable"]
pub type FACCEN_R = crate::BitReader;
#[doc = "Field `FACCEN` writer - Flash access enable"]
pub type FACCEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BURSTEN` reader - Burst enable bit"]
pub type BURSTEN_R = crate::BitReader;
#[doc = "Field `BURSTEN` writer - Burst enable bit"]
pub type BURSTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAITPOL` reader - Wait signal polarity bit"]
pub type WAITPOL_R = crate::BitReader;
#[doc = "Field `WAITPOL` writer - Wait signal polarity bit"]
pub type WAITPOL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WRAPMOD` reader - Wrapped burst mode support"]
pub type WRAPMOD_R = crate::BitReader;
#[doc = "Field `WRAPMOD` writer - Wrapped burst mode support"]
pub type WRAPMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAITCFG` reader - Wait timing configuration"]
pub type WAITCFG_R = crate::BitReader;
#[doc = "Field `WAITCFG` writer - Wait timing configuration"]
pub type WAITCFG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WREN` reader - Write enable bit"]
pub type WREN_R = crate::BitReader;
#[doc = "Field `WREN` writer - Write enable bit"]
pub type WREN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `WAITEN` reader - Wait enable bit"]
pub type WAITEN_R = crate::BitReader;
#[doc = "Field `WAITEN` writer - Wait enable bit"]
pub type WAITEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTMOD` reader - Extended mode enable"]
pub type EXTMOD_R = crate::BitReader;
#[doc = "Field `EXTMOD` writer - Extended mode enable"]
pub type EXTMOD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ASYNCWAIT` reader - Wait signal during asynchronous transfers"]
pub type ASYNCWAIT_R = crate::BitReader;
#[doc = "Field `ASYNCWAIT` writer - Wait signal during asynchronous transfers"]
pub type ASYNCWAIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CBURSTRW` reader - Write burst enable"]
pub type CBURSTRW_R = crate::BitReader;
#[doc = "Field `CBURSTRW` writer - Write burst enable"]
pub type CBURSTRW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Memory bank enable bit"]
    #[inline(always)]
    pub fn mbken(&self) -> MBKEN_R {
        MBKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address/data multiplexing enable bit"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Memory type"]
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Memory databus width"]
    #[inline(always)]
    pub fn mwid(&self) -> MWID_R {
        MWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Flash access enable"]
    #[inline(always)]
    pub fn faccen(&self) -> FACCEN_R {
        FACCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Burst enable bit"]
    #[inline(always)]
    pub fn bursten(&self) -> BURSTEN_R {
        BURSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wait signal polarity bit"]
    #[inline(always)]
    pub fn waitpol(&self) -> WAITPOL_R {
        WAITPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wrapped burst mode support"]
    #[inline(always)]
    pub fn wrapmod(&self) -> WRAPMOD_R {
        WRAPMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wait timing configuration"]
    #[inline(always)]
    pub fn waitcfg(&self) -> WAITCFG_R {
        WAITCFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write enable bit"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wait enable bit"]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Extended mode enable"]
    #[inline(always)]
    pub fn extmod(&self) -> EXTMOD_R {
        EXTMOD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wait signal during asynchronous transfers"]
    #[inline(always)]
    pub fn asyncwait(&self) -> ASYNCWAIT_R {
        ASYNCWAIT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Write burst enable"]
    #[inline(always)]
    pub fn cburstrw(&self) -> CBURSTRW_R {
        CBURSTRW_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Memory bank enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn mbken(&mut self) -> MBKEN_W<BCR1_SPEC, 0> {
        MBKEN_W::new(self)
    }
    #[doc = "Bit 1 - Address/data multiplexing enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn muxen(&mut self) -> MUXEN_W<BCR1_SPEC, 1> {
        MUXEN_W::new(self)
    }
    #[doc = "Bits 2:3 - Memory type"]
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<BCR1_SPEC, 2> {
        MTYP_W::new(self)
    }
    #[doc = "Bits 4:5 - Memory databus width"]
    #[inline(always)]
    #[must_use]
    pub fn mwid(&mut self) -> MWID_W<BCR1_SPEC, 4> {
        MWID_W::new(self)
    }
    #[doc = "Bit 6 - Flash access enable"]
    #[inline(always)]
    #[must_use]
    pub fn faccen(&mut self) -> FACCEN_W<BCR1_SPEC, 6> {
        FACCEN_W::new(self)
    }
    #[doc = "Bit 8 - Burst enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn bursten(&mut self) -> BURSTEN_W<BCR1_SPEC, 8> {
        BURSTEN_W::new(self)
    }
    #[doc = "Bit 9 - Wait signal polarity bit"]
    #[inline(always)]
    #[must_use]
    pub fn waitpol(&mut self) -> WAITPOL_W<BCR1_SPEC, 9> {
        WAITPOL_W::new(self)
    }
    #[doc = "Bit 10 - Wrapped burst mode support"]
    #[inline(always)]
    #[must_use]
    pub fn wrapmod(&mut self) -> WRAPMOD_W<BCR1_SPEC, 10> {
        WRAPMOD_W::new(self)
    }
    #[doc = "Bit 11 - Wait timing configuration"]
    #[inline(always)]
    #[must_use]
    pub fn waitcfg(&mut self) -> WAITCFG_W<BCR1_SPEC, 11> {
        WAITCFG_W::new(self)
    }
    #[doc = "Bit 12 - Write enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WREN_W<BCR1_SPEC, 12> {
        WREN_W::new(self)
    }
    #[doc = "Bit 13 - Wait enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<BCR1_SPEC, 13> {
        WAITEN_W::new(self)
    }
    #[doc = "Bit 14 - Extended mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn extmod(&mut self) -> EXTMOD_W<BCR1_SPEC, 14> {
        EXTMOD_W::new(self)
    }
    #[doc = "Bit 15 - Wait signal during asynchronous transfers"]
    #[inline(always)]
    #[must_use]
    pub fn asyncwait(&mut self) -> ASYNCWAIT_W<BCR1_SPEC, 15> {
        ASYNCWAIT_W::new(self)
    }
    #[doc = "Bit 19 - Write burst enable"]
    #[inline(always)]
    #[must_use]
    pub fn cburstrw(&mut self) -> CBURSTRW_W<BCR1_SPEC, 19> {
        CBURSTRW_W::new(self)
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
#[doc = "SRAM/NOR-Flash chip-select control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BCR1_SPEC;
impl crate::RegisterSpec for BCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr1::R`](R) reader structure"]
impl crate::Readable for BCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bcr1::W`](W) writer structure"]
impl crate::Writable for BCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCR1 to value 0x30d0"]
impl crate::Resettable for BCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x30d0;
}

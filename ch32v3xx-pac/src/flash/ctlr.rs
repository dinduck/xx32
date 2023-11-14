#[doc = "Register `CTLR` reader"]
pub type R = crate::R<CTLR_SPEC>;
#[doc = "Register `CTLR` writer"]
pub type W = crate::W<CTLR_SPEC>;
#[doc = "Field `PG` reader - Programming"]
pub type PG_R = crate::BitReader;
#[doc = "Field `PG` writer - Programming"]
pub type PG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PER` reader - Page Erase"]
pub type PER_R = crate::BitReader;
#[doc = "Field `PER` writer - Page Erase"]
pub type PER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MER` reader - Mass Erase"]
pub type MER_R = crate::BitReader;
#[doc = "Field `MER` writer - Mass Erase"]
pub type MER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OBPG` reader - Option byte programming"]
pub type OBPG_R = crate::BitReader;
#[doc = "Field `OBPG` writer - Option byte programming"]
pub type OBPG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OBER` reader - Option byte erase"]
pub type OBER_R = crate::BitReader;
#[doc = "Field `OBER` writer - Option byte erase"]
pub type OBER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STRT` reader - Start"]
pub type STRT_R = crate::BitReader;
#[doc = "Field `STRT` writer - Start"]
pub type STRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OBWRE` reader - Option bytes write enable"]
pub type OBWRE_R = crate::BitReader;
#[doc = "Field `OBWRE` writer - Option bytes write enable"]
pub type OBWRE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EOPIE` reader - End of operation interrupt enable"]
pub type EOPIE_R = crate::BitReader;
#[doc = "Field `EOPIE` writer - End of operation interrupt enable"]
pub type EOPIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FLOCK` reader - Fast programmable lock"]
pub type FLOCK_R = crate::BitReader;
#[doc = "Field `FLOCK` writer - Fast programmable lock"]
pub type FLOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAGE_PG` reader - Fast programming"]
pub type PAGE_PG_R = crate::BitReader;
#[doc = "Field `PAGE_PG` writer - Fast programming"]
pub type PAGE_PG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAGE_ER` reader - Fast erase"]
pub type PAGE_ER_R = crate::BitReader;
#[doc = "Field `PAGE_ER` writer - Fast erase"]
pub type PAGE_ER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BER32` reader - Block Erase 32K"]
pub type BER32_R = crate::BitReader;
#[doc = "Field `BER32` writer - Block Erase 32K"]
pub type BER32_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BER64` reader - Block Erase 64K"]
pub type BER64_R = crate::BitReader;
#[doc = "Field `BER64` writer - Block Erase 64K"]
pub type BER64_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PGSTART` reader - Page Programming Start"]
pub type PGSTART_R = crate::BitReader;
#[doc = "Field `PGSTART` writer - Page Programming Start"]
pub type PGSTART_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSENACT` reader - Reset Flash Enhance read mode"]
pub type RSENACT_R = crate::BitReader;
#[doc = "Field `RSENACT` writer - Reset Flash Enhance read mode"]
pub type RSENACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENHANCEMODE` reader - Flash Enhance read mode"]
pub type ENHANCEMODE_R = crate::BitReader;
#[doc = "Field `ENHANCEMODE` writer - Flash Enhance read mode"]
pub type ENHANCEMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCKMODE` reader - Flash SCK mode"]
pub type SCKMODE_R = crate::BitReader;
#[doc = "Field `SCKMODE` writer - Flash SCK mode"]
pub type SCKMODE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass Erase"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    pub fn obpg(&self) -> OBPG_R {
        OBPG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    pub fn ober(&self) -> OBER_R {
        OBER_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    pub fn obwre(&self) -> OBWRE_R {
        OBWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    pub fn eopie(&self) -> EOPIE_R {
        EOPIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast programmable lock"]
    #[inline(always)]
    pub fn flock(&self) -> FLOCK_R {
        FLOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast programming"]
    #[inline(always)]
    pub fn page_pg(&self) -> PAGE_PG_R {
        PAGE_PG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast erase"]
    #[inline(always)]
    pub fn page_er(&self) -> PAGE_ER_R {
        PAGE_ER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Block Erase 32K"]
    #[inline(always)]
    pub fn ber32(&self) -> BER32_R {
        BER32_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Block Erase 64K"]
    #[inline(always)]
    pub fn ber64(&self) -> BER64_R {
        BER64_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Page Programming Start"]
    #[inline(always)]
    pub fn pgstart(&self) -> PGSTART_R {
        PGSTART_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset Flash Enhance read mode"]
    #[inline(always)]
    pub fn rsenact(&self) -> RSENACT_R {
        RSENACT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Flash Enhance read mode"]
    #[inline(always)]
    pub fn enhancemode(&self) -> ENHANCEMODE_R {
        ENHANCEMODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Flash SCK mode"]
    #[inline(always)]
    pub fn sckmode(&self) -> SCKMODE_R {
        SCKMODE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Programming"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PG_W<CTLR_SPEC, 0> {
        PG_W::new(self)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<CTLR_SPEC, 1> {
        PER_W::new(self)
    }
    #[doc = "Bit 2 - Mass Erase"]
    #[inline(always)]
    #[must_use]
    pub fn mer(&mut self) -> MER_W<CTLR_SPEC, 2> {
        MER_W::new(self)
    }
    #[doc = "Bit 4 - Option byte programming"]
    #[inline(always)]
    #[must_use]
    pub fn obpg(&mut self) -> OBPG_W<CTLR_SPEC, 4> {
        OBPG_W::new(self)
    }
    #[doc = "Bit 5 - Option byte erase"]
    #[inline(always)]
    #[must_use]
    pub fn ober(&mut self) -> OBER_W<CTLR_SPEC, 5> {
        OBER_W::new(self)
    }
    #[doc = "Bit 6 - Start"]
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<CTLR_SPEC, 6> {
        STRT_W::new(self)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CTLR_SPEC, 7> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 9 - Option bytes write enable"]
    #[inline(always)]
    #[must_use]
    pub fn obwre(&mut self) -> OBWRE_W<CTLR_SPEC, 9> {
        OBWRE_W::new(self)
    }
    #[doc = "Bit 10 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CTLR_SPEC, 10> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 12 - End of operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eopie(&mut self) -> EOPIE_W<CTLR_SPEC, 12> {
        EOPIE_W::new(self)
    }
    #[doc = "Bit 15 - Fast programmable lock"]
    #[inline(always)]
    #[must_use]
    pub fn flock(&mut self) -> FLOCK_W<CTLR_SPEC, 15> {
        FLOCK_W::new(self)
    }
    #[doc = "Bit 16 - Fast programming"]
    #[inline(always)]
    #[must_use]
    pub fn page_pg(&mut self) -> PAGE_PG_W<CTLR_SPEC, 16> {
        PAGE_PG_W::new(self)
    }
    #[doc = "Bit 17 - Fast erase"]
    #[inline(always)]
    #[must_use]
    pub fn page_er(&mut self) -> PAGE_ER_W<CTLR_SPEC, 17> {
        PAGE_ER_W::new(self)
    }
    #[doc = "Bit 18 - Block Erase 32K"]
    #[inline(always)]
    #[must_use]
    pub fn ber32(&mut self) -> BER32_W<CTLR_SPEC, 18> {
        BER32_W::new(self)
    }
    #[doc = "Bit 19 - Block Erase 64K"]
    #[inline(always)]
    #[must_use]
    pub fn ber64(&mut self) -> BER64_W<CTLR_SPEC, 19> {
        BER64_W::new(self)
    }
    #[doc = "Bit 21 - Page Programming Start"]
    #[inline(always)]
    #[must_use]
    pub fn pgstart(&mut self) -> PGSTART_W<CTLR_SPEC, 21> {
        PGSTART_W::new(self)
    }
    #[doc = "Bit 22 - Reset Flash Enhance read mode"]
    #[inline(always)]
    #[must_use]
    pub fn rsenact(&mut self) -> RSENACT_W<CTLR_SPEC, 22> {
        RSENACT_W::new(self)
    }
    #[doc = "Bit 24 - Flash Enhance read mode"]
    #[inline(always)]
    #[must_use]
    pub fn enhancemode(&mut self) -> ENHANCEMODE_W<CTLR_SPEC, 24> {
        ENHANCEMODE_W::new(self)
    }
    #[doc = "Bit 25 - Flash SCK mode"]
    #[inline(always)]
    #[must_use]
    pub fn sckmode(&mut self) -> SCKMODE_W<CTLR_SPEC, 25> {
        SCKMODE_W::new(self)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTLR_SPEC;
impl crate::RegisterSpec for CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlr::R`](R) reader structure"]
impl crate::Readable for CTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctlr::W`](W) writer structure"]
impl crate::Writable for CTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTLR to value 0x80"]
impl crate::Resettable for CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}

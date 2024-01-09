#[doc = "Register `RDTIMING3` reader"]
pub type R = crate::R<RDTIMING3_SPEC>;
#[doc = "Register `RDTIMING3` writer"]
pub type W = crate::W<RDTIMING3_SPEC>;
#[doc = "Field `RDSETUP` reader - Read Setup Time"]
pub type RDSETUP_R = crate::FieldReader;
#[doc = "Field `RDSETUP` writer - Read Setup Time"]
pub type RDSETUP_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDSTRB` reader - Read Strobe Time"]
pub type RDSTRB_R = crate::FieldReader;
#[doc = "Field `RDSTRB` writer - Read Strobe Time"]
pub type RDSTRB_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RDHOLD` reader - Read Hold Time"]
pub type RDHOLD_R = crate::FieldReader;
#[doc = "Field `RDHOLD` writer - Read Hold Time"]
pub type RDHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HALFRE` reader - Half Cycle REn Strobe Duration Enable"]
pub type HALFRE_R = crate::BitReader;
#[doc = "Field `HALFRE` writer - Half Cycle REn Strobe Duration Enable"]
pub type HALFRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PREFETCH` reader - Prefetch Enable"]
pub type PREFETCH_R = crate::BitReader;
#[doc = "Field `PREFETCH` writer - Prefetch Enable"]
pub type PREFETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGEMODE` reader - Page Mode Access Enable"]
pub type PAGEMODE_R = crate::BitReader;
#[doc = "Field `PAGEMODE` writer - Page Mode Access Enable"]
pub type PAGEMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Read Setup Time"]
    #[inline(always)]
    pub fn rdsetup(&self) -> RDSETUP_R {
        RDSETUP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Read Strobe Time"]
    #[inline(always)]
    pub fn rdstrb(&self) -> RDSTRB_R {
        RDSTRB_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - Read Hold Time"]
    #[inline(always)]
    pub fn rdhold(&self) -> RDHOLD_R {
        RDHOLD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 28 - Half Cycle REn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfre(&self) -> HALFRE_R {
        HALFRE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Prefetch Enable"]
    #[inline(always)]
    pub fn prefetch(&self) -> PREFETCH_R {
        PREFETCH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Page Mode Access Enable"]
    #[inline(always)]
    pub fn pagemode(&self) -> PAGEMODE_R {
        PAGEMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn rdsetup(&mut self) -> RDSETUP_W<RDTIMING3_SPEC> {
        RDSETUP_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Read Strobe Time"]
    #[inline(always)]
    #[must_use]
    pub fn rdstrb(&mut self) -> RDSTRB_W<RDTIMING3_SPEC> {
        RDSTRB_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - Read Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn rdhold(&mut self) -> RDHOLD_W<RDTIMING3_SPEC> {
        RDHOLD_W::new(self, 16)
    }
    #[doc = "Bit 28 - Half Cycle REn Strobe Duration Enable"]
    #[inline(always)]
    #[must_use]
    pub fn halfre(&mut self) -> HALFRE_W<RDTIMING3_SPEC> {
        HALFRE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Prefetch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prefetch(&mut self) -> PREFETCH_W<RDTIMING3_SPEC> {
        PREFETCH_W::new(self, 29)
    }
    #[doc = "Bit 30 - Page Mode Access Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pagemode(&mut self) -> PAGEMODE_W<RDTIMING3_SPEC> {
        PAGEMODE_W::new(self, 30)
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
#[doc = "Read Timing Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdtiming3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdtiming3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RDTIMING3_SPEC;
impl crate::RegisterSpec for RDTIMING3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdtiming3::R`](R) reader structure"]
impl crate::Readable for RDTIMING3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rdtiming3::W`](W) writer structure"]
impl crate::Writable for RDTIMING3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDTIMING3 to value 0x0007_7f07"]
impl crate::Resettable for RDTIMING3_SPEC {
    const RESET_VALUE: u32 = 0x0007_7f07;
}

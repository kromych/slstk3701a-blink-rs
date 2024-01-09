#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `OF` reader - OF Interrupt Enable"]
pub type OF_R = crate::BitReader;
#[doc = "Field `OF` writer - OF Interrupt Enable"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` reader - UF Interrupt Enable"]
pub type UF_R = crate::BitReader;
#[doc = "Field `UF` writer - UF Interrupt Enable"]
pub type UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` reader - DIRCHG Interrupt Enable"]
pub type DIRCHG_R = crate::BitReader;
#[doc = "Field `DIRCHG` writer - DIRCHG Interrupt Enable"]
pub type DIRCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` reader - CC0 Interrupt Enable"]
pub type CC0_R = crate::BitReader;
#[doc = "Field `CC0` writer - CC0 Interrupt Enable"]
pub type CC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` reader - CC1 Interrupt Enable"]
pub type CC1_R = crate::BitReader;
#[doc = "Field `CC1` writer - CC1 Interrupt Enable"]
pub type CC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` reader - CC2 Interrupt Enable"]
pub type CC2_R = crate::BitReader;
#[doc = "Field `CC2` writer - CC2 Interrupt Enable"]
pub type CC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3` reader - CC3 Interrupt Enable"]
pub type CC3_R = crate::BitReader;
#[doc = "Field `CC3` writer - CC3 Interrupt Enable"]
pub type CC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF0` reader - ICBOF0 Interrupt Enable"]
pub type ICBOF0_R = crate::BitReader;
#[doc = "Field `ICBOF0` writer - ICBOF0 Interrupt Enable"]
pub type ICBOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF1` reader - ICBOF1 Interrupt Enable"]
pub type ICBOF1_R = crate::BitReader;
#[doc = "Field `ICBOF1` writer - ICBOF1 Interrupt Enable"]
pub type ICBOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF2` reader - ICBOF2 Interrupt Enable"]
pub type ICBOF2_R = crate::BitReader;
#[doc = "Field `ICBOF2` writer - ICBOF2 Interrupt Enable"]
pub type ICBOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF3` reader - ICBOF3 Interrupt Enable"]
pub type ICBOF3_R = crate::BitReader;
#[doc = "Field `ICBOF3` writer - ICBOF3 Interrupt Enable"]
pub type ICBOF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIRCHG Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CC3 Interrupt Enable"]
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ICBOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof0(&self) -> ICBOF0_R {
        ICBOF0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ICBOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof1(&self) -> ICBOF1_R {
        ICBOF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICBOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof2(&self) -> ICBOF2_R {
        ICBOF2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ICBOF3 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof3(&self) -> ICBOF3_R {
        ICBOF3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IEN_SPEC> {
        OF_W::new(self, 0)
    }
    #[doc = "Bit 1 - UF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<IEN_SPEC> {
        UF_W::new(self, 1)
    }
    #[doc = "Bit 2 - DIRCHG Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DIRCHG_W<IEN_SPEC> {
        DIRCHG_W::new(self, 2)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> CC0_W<IEN_SPEC> {
        CC0_W::new(self, 4)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<IEN_SPEC> {
        CC1_W::new(self, 5)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<IEN_SPEC> {
        CC2_W::new(self, 6)
    }
    #[doc = "Bit 7 - CC3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc3(&mut self) -> CC3_W<IEN_SPEC> {
        CC3_W::new(self, 7)
    }
    #[doc = "Bit 8 - ICBOF0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbof0(&mut self) -> ICBOF0_W<IEN_SPEC> {
        ICBOF0_W::new(self, 8)
    }
    #[doc = "Bit 9 - ICBOF1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbof1(&mut self) -> ICBOF1_W<IEN_SPEC> {
        ICBOF1_W::new(self, 9)
    }
    #[doc = "Bit 10 - ICBOF2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbof2(&mut self) -> ICBOF2_W<IEN_SPEC> {
        ICBOF2_W::new(self, 10)
    }
    #[doc = "Bit 11 - ICBOF3 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icbof3(&mut self) -> ICBOF3_W<IEN_SPEC> {
        ICBOF3_W::new(self, 11)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: u32 = 0;
}

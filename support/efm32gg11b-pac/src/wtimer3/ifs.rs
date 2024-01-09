#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `OF` writer - Set OF Interrupt Flag"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UF` writer - Set UF Interrupt Flag"]
pub type UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCHG` writer - Set DIRCHG Interrupt Flag"]
pub type DIRCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0` writer - Set CC0 Interrupt Flag"]
pub type CC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1` writer - Set CC1 Interrupt Flag"]
pub type CC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2` writer - Set CC2 Interrupt Flag"]
pub type CC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3` writer - Set CC3 Interrupt Flag"]
pub type CC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF0` writer - Set ICBOF0 Interrupt Flag"]
pub type ICBOF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF1` writer - Set ICBOF1 Interrupt Flag"]
pub type ICBOF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF2` writer - Set ICBOF2 Interrupt Flag"]
pub type ICBOF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICBOF3` writer - Set ICBOF3 Interrupt Flag"]
pub type ICBOF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set OF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IFS_SPEC> {
        OF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set UF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<IFS_SPEC> {
        UF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set DIRCHG Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirchg(&mut self) -> DIRCHG_W<IFS_SPEC> {
        DIRCHG_W::new(self, 2)
    }
    #[doc = "Bit 4 - Set CC0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc0(&mut self) -> CC0_W<IFS_SPEC> {
        CC0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set CC1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc1(&mut self) -> CC1_W<IFS_SPEC> {
        CC1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set CC2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc2(&mut self) -> CC2_W<IFS_SPEC> {
        CC2_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set CC3 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cc3(&mut self) -> CC3_W<IFS_SPEC> {
        CC3_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set ICBOF0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icbof0(&mut self) -> ICBOF0_W<IFS_SPEC> {
        ICBOF0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set ICBOF1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icbof1(&mut self) -> ICBOF1_W<IFS_SPEC> {
        ICBOF1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set ICBOF2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icbof2(&mut self) -> ICBOF2_W<IFS_SPEC> {
        ICBOF2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set ICBOF3 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn icbof3(&mut self) -> ICBOF3_W<IFS_SPEC> {
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
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: u32 = 0;
}

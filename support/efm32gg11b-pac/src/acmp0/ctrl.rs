#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EN` reader - Analog Comparator Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Analog Comparator Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INACTVAL` reader - Inactive Value"]
pub type INACTVAL_R = crate::BitReader;
#[doc = "Field `INACTVAL` writer - Inactive Value"]
pub type INACTVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOINV` reader - Comparator GPIO Output Invert"]
pub type GPIOINV_R = crate::BitReader;
#[doc = "Field `GPIOINV` writer - Comparator GPIO Output Invert"]
pub type GPIOINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTXMASTERDIS` reader - APORT Bus X Master Disable"]
pub type APORTXMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORTXMASTERDIS` writer - APORT Bus X Master Disable"]
pub type APORTXMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTYMASTERDIS` reader - APORT Bus Y Master Disable"]
pub type APORTYMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORTYMASTERDIS` writer - APORT Bus Y Master Disable"]
pub type APORTYMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APORTVMASTERDIS` reader - APORT Bus Master Disable for Bus Selected By VASEL"]
pub type APORTVMASTERDIS_R = crate::BitReader;
#[doc = "Field `APORTVMASTERDIS` writer - APORT Bus Master Disable for Bus Selected By VASEL"]
pub type APORTVMASTERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSEL` reader - Power Select"]
pub type PWRSEL_R = crate::FieldReader<PWRSEL_A>;
#[doc = "Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRSEL_A {
    #[doc = "0: AVDD supply"]
    AVDD = 0,
    #[doc = "1: DVDD supply"]
    DVDD = 1,
    #[doc = "2: IOVDD/IOVDD0 supply"]
    IOVDD0 = 2,
    #[doc = "4: IOVDD1 supply (if part has two I/O voltages)"]
    IOVDD1 = 4,
}
impl From<PWRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRSEL_A {
    type Ux = u8;
}
impl PWRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWRSEL_A> {
        match self.bits {
            0 => Some(PWRSEL_A::AVDD),
            1 => Some(PWRSEL_A::DVDD),
            2 => Some(PWRSEL_A::IOVDD0),
            4 => Some(PWRSEL_A::IOVDD1),
            _ => None,
        }
    }
    #[doc = "AVDD supply"]
    #[inline(always)]
    pub fn is_avdd(&self) -> bool {
        *self == PWRSEL_A::AVDD
    }
    #[doc = "DVDD supply"]
    #[inline(always)]
    pub fn is_dvdd(&self) -> bool {
        *self == PWRSEL_A::DVDD
    }
    #[doc = "IOVDD/IOVDD0 supply"]
    #[inline(always)]
    pub fn is_iovdd0(&self) -> bool {
        *self == PWRSEL_A::IOVDD0
    }
    #[doc = "IOVDD1 supply (if part has two I/O voltages)"]
    #[inline(always)]
    pub fn is_iovdd1(&self) -> bool {
        *self == PWRSEL_A::IOVDD1
    }
}
#[doc = "Field `PWRSEL` writer - Power Select"]
pub type PWRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PWRSEL_A>;
impl<'a, REG> PWRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AVDD supply"]
    #[inline(always)]
    pub fn avdd(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSEL_A::AVDD)
    }
    #[doc = "DVDD supply"]
    #[inline(always)]
    pub fn dvdd(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSEL_A::DVDD)
    }
    #[doc = "IOVDD/IOVDD0 supply"]
    #[inline(always)]
    pub fn iovdd0(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSEL_A::IOVDD0)
    }
    #[doc = "IOVDD1 supply (if part has two I/O voltages)"]
    #[inline(always)]
    pub fn iovdd1(self) -> &'a mut crate::W<REG> {
        self.variant(PWRSEL_A::IOVDD1)
    }
}
#[doc = "Field `ACCURACY` reader - ACMP Accuracy Mode"]
pub type ACCURACY_R = crate::BitReader;
#[doc = "Field `ACCURACY` writer - ACMP Accuracy Mode"]
pub type ACCURACY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUTRANGE` reader - Input Range"]
pub type INPUTRANGE_R = crate::FieldReader<INPUTRANGE_A>;
#[doc = "Input Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INPUTRANGE_A {
    #[doc = "0: Setting when the input can be from 0 to ACMPVDD."]
    FULL = 0,
    #[doc = "1: Setting when the input will always be greater than ACMPVDD/2."]
    GTVDDDIV2 = 1,
    #[doc = "2: Setting when the input will always be less than ACMPVDD/2."]
    LTVDDDIV2 = 2,
}
impl From<INPUTRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTRANGE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INPUTRANGE_A {
    type Ux = u8;
}
impl INPUTRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<INPUTRANGE_A> {
        match self.bits {
            0 => Some(INPUTRANGE_A::FULL),
            1 => Some(INPUTRANGE_A::GTVDDDIV2),
            2 => Some(INPUTRANGE_A::LTVDDDIV2),
            _ => None,
        }
    }
    #[doc = "Setting when the input can be from 0 to ACMPVDD."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == INPUTRANGE_A::FULL
    }
    #[doc = "Setting when the input will always be greater than ACMPVDD/2."]
    #[inline(always)]
    pub fn is_gtvdddiv2(&self) -> bool {
        *self == INPUTRANGE_A::GTVDDDIV2
    }
    #[doc = "Setting when the input will always be less than ACMPVDD/2."]
    #[inline(always)]
    pub fn is_ltvdddiv2(&self) -> bool {
        *self == INPUTRANGE_A::LTVDDDIV2
    }
}
#[doc = "Field `INPUTRANGE` writer - Input Range"]
pub type INPUTRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, INPUTRANGE_A>;
impl<'a, REG> INPUTRANGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Setting when the input can be from 0 to ACMPVDD."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTRANGE_A::FULL)
    }
    #[doc = "Setting when the input will always be greater than ACMPVDD/2."]
    #[inline(always)]
    pub fn gtvdddiv2(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTRANGE_A::GTVDDDIV2)
    }
    #[doc = "Setting when the input will always be less than ACMPVDD/2."]
    #[inline(always)]
    pub fn ltvdddiv2(self) -> &'a mut crate::W<REG> {
        self.variant(INPUTRANGE_A::LTVDDDIV2)
    }
}
#[doc = "Field `IRISE` reader - Rising Edge Interrupt Sense"]
pub type IRISE_R = crate::BitReader;
#[doc = "Field `IRISE` writer - Rising Edge Interrupt Sense"]
pub type IRISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFALL` reader - Falling Edge Interrupt Sense"]
pub type IFALL_R = crate::BitReader;
#[doc = "Field `IFALL` writer - Falling Edge Interrupt Sense"]
pub type IFALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIASPROG` reader - Bias Configuration"]
pub type BIASPROG_R = crate::FieldReader;
#[doc = "Field `BIASPROG` writer - Bias Configuration"]
pub type BIASPROG_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FULLBIAS` reader - Full Bias Current"]
pub type FULLBIAS_R = crate::BitReader;
#[doc = "Field `FULLBIAS` writer - Full Bias Current"]
pub type FULLBIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    pub fn inactval(&self) -> INACTVAL_R {
        INACTVAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    pub fn gpioinv(&self) -> GPIOINV_R {
        GPIOINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - APORT Bus X Master Disable"]
    #[inline(always)]
    pub fn aportxmasterdis(&self) -> APORTXMASTERDIS_R {
        APORTXMASTERDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - APORT Bus Y Master Disable"]
    #[inline(always)]
    pub fn aportymasterdis(&self) -> APORTYMASTERDIS_R {
        APORTYMASTERDIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - APORT Bus Master Disable for Bus Selected By VASEL"]
    #[inline(always)]
    pub fn aportvmasterdis(&self) -> APORTVMASTERDIS_R {
        APORTVMASTERDIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Power Select"]
    #[inline(always)]
    pub fn pwrsel(&self) -> PWRSEL_R {
        PWRSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - ACMP Accuracy Mode"]
    #[inline(always)]
    pub fn accuracy(&self) -> ACCURACY_R {
        ACCURACY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Input Range"]
    #[inline(always)]
    pub fn inputrange(&self) -> INPUTRANGE_R {
        INPUTRANGE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    pub fn irise(&self) -> IRISE_R {
        IRISE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    pub fn ifall(&self) -> IFALL_R {
        IFALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Bias Configuration"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    pub fn fullbias(&self) -> FULLBIAS_R {
        FULLBIAS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CTRL_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 2 - Inactive Value"]
    #[inline(always)]
    #[must_use]
    pub fn inactval(&mut self) -> INACTVAL_W<CTRL_SPEC> {
        INACTVAL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Comparator GPIO Output Invert"]
    #[inline(always)]
    #[must_use]
    pub fn gpioinv(&mut self) -> GPIOINV_W<CTRL_SPEC> {
        GPIOINV_W::new(self, 3)
    }
    #[doc = "Bit 8 - APORT Bus X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aportxmasterdis(&mut self) -> APORTXMASTERDIS_W<CTRL_SPEC> {
        APORTXMASTERDIS_W::new(self, 8)
    }
    #[doc = "Bit 9 - APORT Bus Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aportymasterdis(&mut self) -> APORTYMASTERDIS_W<CTRL_SPEC> {
        APORTYMASTERDIS_W::new(self, 9)
    }
    #[doc = "Bit 10 - APORT Bus Master Disable for Bus Selected By VASEL"]
    #[inline(always)]
    #[must_use]
    pub fn aportvmasterdis(&mut self) -> APORTVMASTERDIS_W<CTRL_SPEC> {
        APORTVMASTERDIS_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - Power Select"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsel(&mut self) -> PWRSEL_W<CTRL_SPEC> {
        PWRSEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - ACMP Accuracy Mode"]
    #[inline(always)]
    #[must_use]
    pub fn accuracy(&mut self) -> ACCURACY_W<CTRL_SPEC> {
        ACCURACY_W::new(self, 15)
    }
    #[doc = "Bits 18:19 - Input Range"]
    #[inline(always)]
    #[must_use]
    pub fn inputrange(&mut self) -> INPUTRANGE_W<CTRL_SPEC> {
        INPUTRANGE_W::new(self, 18)
    }
    #[doc = "Bit 20 - Rising Edge Interrupt Sense"]
    #[inline(always)]
    #[must_use]
    pub fn irise(&mut self) -> IRISE_W<CTRL_SPEC> {
        IRISE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Falling Edge Interrupt Sense"]
    #[inline(always)]
    #[must_use]
    pub fn ifall(&mut self) -> IFALL_W<CTRL_SPEC> {
        IFALL_W::new(self, 21)
    }
    #[doc = "Bits 24:29 - Bias Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn biasprog(&mut self) -> BIASPROG_W<CTRL_SPEC> {
        BIASPROG_W::new(self, 24)
    }
    #[doc = "Bit 31 - Full Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn fullbias(&mut self) -> FULLBIAS_W<CTRL_SPEC> {
        FULLBIAS_W::new(self, 31)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0700_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0700_0000;
}

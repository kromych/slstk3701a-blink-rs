#[doc = "Register `TFTCOLORFORMAT` reader"]
pub type R = crate::R<TftcolorformatSpec>;
#[doc = "Register `TFTCOLORFORMAT` writer"]
pub type W = crate::W<TftcolorformatSpec>;
#[doc = "Sprite Pixel Color Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pixel0format {
    #[doc = "0: ARGB data is 0555"]
    Argb0555 = 0,
    #[doc = "1: ARGB data is 0565"]
    Argb0565 = 1,
    #[doc = "2: ARGB data is 0666"]
    Argb0666 = 2,
    #[doc = "3: ARGB data is 0888"]
    Argb0888 = 3,
    #[doc = "4: ARGB data is 5555"]
    Argb5555 = 4,
    #[doc = "5: ARGB data is 6565"]
    Argb6565 = 5,
    #[doc = "6: ARGB data is 6666"]
    Argb6666 = 6,
    #[doc = "7: ARGB data is 8888"]
    Argb8888 = 7,
}
impl From<Pixel0format> for u8 {
    #[inline(always)]
    fn from(variant: Pixel0format) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pixel0format {
    type Ux = u8;
}
impl crate::IsEnum for Pixel0format {}
#[doc = "Field `PIXEL0FORMAT` reader - Sprite Pixel Color Format"]
pub type Pixel0formatR = crate::FieldReader<Pixel0format>;
impl Pixel0formatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pixel0format {
        match self.bits {
            0 => Pixel0format::Argb0555,
            1 => Pixel0format::Argb0565,
            2 => Pixel0format::Argb0666,
            3 => Pixel0format::Argb0888,
            4 => Pixel0format::Argb5555,
            5 => Pixel0format::Argb6565,
            6 => Pixel0format::Argb6666,
            7 => Pixel0format::Argb8888,
            _ => unreachable!(),
        }
    }
    #[doc = "ARGB data is 0555"]
    #[inline(always)]
    pub fn is_argb0555(&self) -> bool {
        *self == Pixel0format::Argb0555
    }
    #[doc = "ARGB data is 0565"]
    #[inline(always)]
    pub fn is_argb0565(&self) -> bool {
        *self == Pixel0format::Argb0565
    }
    #[doc = "ARGB data is 0666"]
    #[inline(always)]
    pub fn is_argb0666(&self) -> bool {
        *self == Pixel0format::Argb0666
    }
    #[doc = "ARGB data is 0888"]
    #[inline(always)]
    pub fn is_argb0888(&self) -> bool {
        *self == Pixel0format::Argb0888
    }
    #[doc = "ARGB data is 5555"]
    #[inline(always)]
    pub fn is_argb5555(&self) -> bool {
        *self == Pixel0format::Argb5555
    }
    #[doc = "ARGB data is 6565"]
    #[inline(always)]
    pub fn is_argb6565(&self) -> bool {
        *self == Pixel0format::Argb6565
    }
    #[doc = "ARGB data is 6666"]
    #[inline(always)]
    pub fn is_argb6666(&self) -> bool {
        *self == Pixel0format::Argb6666
    }
    #[doc = "ARGB data is 8888"]
    #[inline(always)]
    pub fn is_argb8888(&self) -> bool {
        *self == Pixel0format::Argb8888
    }
}
#[doc = "Field `PIXEL0FORMAT` writer - Sprite Pixel Color Format"]
pub type Pixel0formatW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pixel0format, crate::Safe>;
impl<'a, REG> Pixel0formatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ARGB data is 0555"]
    #[inline(always)]
    pub fn argb0555(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel0format::Argb0555)
    }
    #[doc = "ARGB data is 0565"]
    #[inline(always)]
    pub fn argb0565(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel0format::Argb0565)
    }
    #[doc = "ARGB data is 0666"]
    #[inline(always)]
    pub fn argb0666(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel0format::Argb0666)
    }
    #[doc = "ARGB data is 0888"]
    #[inline(always)]
    pub fn argb0888(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel0format::Argb0888)
    }
    #[doc = "ARGB data is 5555"]
    #[inline(always)]
    pub fn argb5555(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel0format::Argb5555)
    }
    #[doc = "ARGB data is 6565"]
    #[inline(always)]
    pub fn argb6565(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel0format::Argb6565)
    }
    #[doc = "ARGB data is 6666"]
    #[inline(always)]
    pub fn argb6666(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel0format::Argb6666)
    }
    #[doc = "ARGB data is 8888"]
    #[inline(always)]
    pub fn argb8888(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel0format::Argb8888)
    }
}
#[doc = "Source and Destination Pixel Color Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pixel1format {
    #[doc = "0: RGB data is 555"]
    Rgb555 = 0,
    #[doc = "1: RGB data is 565"]
    Rgb565 = 1,
    #[doc = "2: RGB data is 666"]
    Rgb666 = 2,
    #[doc = "3: RGB data is 888"]
    Rgb888 = 3,
}
impl From<Pixel1format> for u8 {
    #[inline(always)]
    fn from(variant: Pixel1format) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pixel1format {
    type Ux = u8;
}
impl crate::IsEnum for Pixel1format {}
#[doc = "Field `PIXEL1FORMAT` reader - Source and Destination Pixel Color Format"]
pub type Pixel1formatR = crate::FieldReader<Pixel1format>;
impl Pixel1formatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pixel1format {
        match self.bits {
            0 => Pixel1format::Rgb555,
            1 => Pixel1format::Rgb565,
            2 => Pixel1format::Rgb666,
            3 => Pixel1format::Rgb888,
            _ => unreachable!(),
        }
    }
    #[doc = "RGB data is 555"]
    #[inline(always)]
    pub fn is_rgb555(&self) -> bool {
        *self == Pixel1format::Rgb555
    }
    #[doc = "RGB data is 565"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        *self == Pixel1format::Rgb565
    }
    #[doc = "RGB data is 666"]
    #[inline(always)]
    pub fn is_rgb666(&self) -> bool {
        *self == Pixel1format::Rgb666
    }
    #[doc = "RGB data is 888"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        *self == Pixel1format::Rgb888
    }
}
#[doc = "Field `PIXEL1FORMAT` writer - Source and Destination Pixel Color Format"]
pub type Pixel1formatW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pixel1format, crate::Safe>;
impl<'a, REG> Pixel1formatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGB data is 555"]
    #[inline(always)]
    pub fn rgb555(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel1format::Rgb555)
    }
    #[doc = "RGB data is 565"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel1format::Rgb565)
    }
    #[doc = "RGB data is 666"]
    #[inline(always)]
    pub fn rgb666(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel1format::Rgb666)
    }
    #[doc = "RGB data is 888"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut crate::W<REG> {
        self.variant(Pixel1format::Rgb888)
    }
}
impl R {
    #[doc = "Bits 0:2 - Sprite Pixel Color Format"]
    #[inline(always)]
    pub fn pixel0format(&self) -> Pixel0formatR {
        Pixel0formatR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Source and Destination Pixel Color Format"]
    #[inline(always)]
    pub fn pixel1format(&self) -> Pixel1formatR {
        Pixel1formatR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sprite Pixel Color Format"]
    #[inline(always)]
    pub fn pixel0format(&mut self) -> Pixel0formatW<'_, TftcolorformatSpec> {
        Pixel0formatW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Source and Destination Pixel Color Format"]
    #[inline(always)]
    pub fn pixel1format(&mut self) -> Pixel1formatW<'_, TftcolorformatSpec> {
        Pixel1formatW::new(self, 8)
    }
}
#[doc = "Color Format Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftcolorformat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftcolorformat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftcolorformatSpec;
impl crate::RegisterSpec for TftcolorformatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftcolorformat::R`](R) reader structure"]
impl crate::Readable for TftcolorformatSpec {}
#[doc = "`write(|w| ..)` method takes [`tftcolorformat::W`](W) writer structure"]
impl crate::Writable for TftcolorformatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTCOLORFORMAT to value 0"]
impl crate::Resettable for TftcolorformatSpec {}
